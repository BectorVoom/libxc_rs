//! GGA_K_LC94 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lc94.c`
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
pub fn gga_k_lc94_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_alpha = f64x8::splat(param_alpha);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
    let param_expo = f64x8::splat(param_expo);
    let param_f = f64x8::splat(param_f);
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
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = param_alpha * t32;
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t37 * v_sigma0;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t44 = t38 * t43;
            let t47 = (simd::exp(-t33 * t44 / f64x8::splat(24.0)));
            let t50 = (param_d * t47 + param_c) * t32;
            let t53 = t32 * t32;
            let t54 = f64x8::splat(1.0) / t35;
            let t55 = t53 * t54;
            let t56 = ((v_sigma0).sqrt());
            let t58 = f64x8::splat(1.0) / t40 / v_rho0;
            let t62 = (simd::pow(t55 * t56 * t58 / f64x8::splat(12.0), param_expo));
            let t63 = param_f * t62;
            let t64 = t50 * t44 / f64x8::splat(24.0) - t63;
            let t65 = t55 * t56;
            let t67 = param_b * t53;
            let t72 = (simd::ln(t67 * t54 * t56 * t58 / f64x8::splat(12.0) + ((((t67 * t54 * t56 * t58 / f64x8::splat(12.0)) * (t67 * t54 * t56 * t58 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t73 = t58 * param_a * t72;
            let t76 = f64x8::splat(1.0) + t65 * t73 / f64x8::splat(12.0) + t63;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t64 * t77 + f64x8::splat(1.0);
            let t83 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t79));
            let t84 = (v_rho1).simd_le(dens_threshold);
            let t85 = -t17;
            let t87 = ((t15).select(t12, (t11).select(t16, t85 * t8)));
            let t88 = f64x8::splat(1.0) + t87;
            let t89 = (t88).simd_le(zeta_threshold);
            let t90 = (simd::cbrt(t88));
            let t91 = t90 * t90;
            let t93 = ((t89).select(t24, t91 * t88));
            let t94 = t93 * t30;
            let t95 = t37 * v_sigma2;
            let t96 = v_rho1 * v_rho1;
            let t97 = (simd::cbrt(v_rho1));
            let t98 = t97 * t97;
            let t100 = f64x8::splat(1.0) / t98 / t96;
            let t101 = t95 * t100;
            let t104 = (simd::exp(-t33 * t101 / f64x8::splat(24.0)));
            let t107 = (param_d * t104 + param_c) * t32;
            let t110 = ((v_sigma2).sqrt());
            let t112 = f64x8::splat(1.0) / t97 / v_rho1;
            let t116 = (simd::pow(t55 * t110 * t112 / f64x8::splat(12.0), param_expo));
            let t117 = param_f * t116;
            let t118 = t107 * t101 / f64x8::splat(24.0) - t117;
            let t119 = t55 * t110;
            let t125 = (simd::ln(t67 * t54 * t110 * t112 / f64x8::splat(12.0) + ((((t67 * t54 * t110 * t112 / f64x8::splat(12.0)) * (t67 * t54 * t110 * t112 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t126 = t112 * param_a * t125;
            let t129 = f64x8::splat(1.0) + t119 * t126 / f64x8::splat(12.0) + t117;
            let t130 = f64x8::splat(1.0) / t129;
            let t132 = t118 * t130 + f64x8::splat(1.0);
            let t136 = ((t84).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t94 * t132));
            let tzk0 = t83 + t136;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
