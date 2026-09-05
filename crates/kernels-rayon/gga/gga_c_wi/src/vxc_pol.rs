//! GGA_C_WI vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wi.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_wi_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
    let param_k = f64x8::splat(param_k);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t2 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t3 = param_b * t2;
            let t4 = v_rho0 + v_rho1;
            let t5 = t4 * t4;
            let t6 = (simd::cbrt(t4));
            let t7 = t6 * t6;
            let t9 = f64x8::splat(1.0) / t7 / t5;
            let t10 = param_k * t2;
            let t12 = (simd::exp(-t10 * t9));
            let t15 = t3 * t9 * t12 + param_a;
            let t16 = f64x8::splat(M_CBRT3);
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t19 = t16 * t18;
            let t20 = f64x8::splat(M_CBRT4);
            let t21 = t20 * t20;
            let t25 = t16 * t16;
            let t26 = f64x8::splat(M_CBRTPI);
            let t28 = ((t2).sqrt());
            let t29 = t28 * t2;
            let t30 = t5 * t5;
            let t31 = f64x8::splat(1.0) / t30;
            let t34 = f64x8::splat(1.0) / t6 / t4;
            let t35 = t28 * t34;
            let t36 = ((t35).sqrt());
            let t41 = f64x8::splat(1.0) + param_d * t20 * t25 * t26 * t36 * t29 * t31 / f64x8::splat(3.0);
            let t45 = param_c + t19 * t21 / t6 * t41 / f64x8::splat(4.0);
            let t46 = f64x8::splat(1.0) / t45;
            let tzk0 = t15 * t46;
            acc_zk = tzk0;
            let t47 = t5 * t4;
            let t49 = f64x8::splat(1.0) / t7 / t47;
            let t52 = t2 * t2;
            let t53 = param_b * t52;
            let t54 = t30 * t5;
            let t56 = f64x8::splat(1.0) / t6 / t54;
            let t61 = f64x8::splat(8.0) / f64x8::splat(3.0) * t53 * t56 * param_k * t12 - f64x8::splat(8.0) / f64x8::splat(3.0) * t3 * t49 * t12;
            let t62 = t4 * t61;
            let t64 = t4 * t15;
            let t65 = t45 * t45;
            let t66 = f64x8::splat(1.0) / t65;
            let t74 = t36 * t2 * t9;
            let t75 = t26 * t74;
            let t76 = t75 * t28;
            let t79 = -t19 * t21 * t34 * t41 / f64x8::splat(12.0) - f64x8::splat(14.0) / f64x8::splat(3.0) * t18 * t9 * param_d * t76;
            let t80 = t66 * t79;
            let tvrho0 = t62 * t46 - t64 * t80 + tzk0;
            acc_vrho_0 = tvrho0;
            let tvrho1 = tvrho0;
            acc_vrho_1 = tvrho1;
            let t84 = t30 * t4;
            let t86 = f64x8::splat(1.0) / t6 / t84;
            let t90 = -t3 * t86 * param_k * t12 + param_b * t9 * t12;
            let t91 = t4 * t90;
            let t93 = f64x8::splat(1.0) / t7;
            let t94 = t93 * t15;
            let t95 = t66 * t18;
            let t96 = t94 * t95;
            let t97 = param_d * t26;
            let t98 = f64x8::splat(1.0) / t28;
            let t99 = t74 * t98;
            let t100 = t97 * t99;
            let t101 = t96 * t100;
            let tvsigma0 = t91 * t46 - f64x8::splat(7.0) / f64x8::splat(4.0) * t101;
            acc_vsigma_0 = tvsigma0;
            let t103 = f64x8::splat(2.0) * t90;
            let t104 = t4 * t103;
            let tvsigma1 = t104 * t46 - f64x8::splat(7.0) / f64x8::splat(2.0) * t101;
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
