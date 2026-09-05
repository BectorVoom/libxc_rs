//! LDA_C_GOMBAS kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`
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
pub fn lda_c_gombas_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        {
            let t1 = v_rho0 + v_rho1;
            let t2 = (simd::cbrt(t1));
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = f64x8::splat(1.0) + f64x8::splat(0.0562) * t3;
            let t7 = f64x8::splat(0.0357) / t5;
            let t8 = t3 + f64x8::splat(2.39);
            let t10 = (simd::ln(t8 * t2));
            let t11 = f64x8::splat(0.0311) * t10;
            let tzk0 = -t7 - t11;
            acc_zk = tzk0;
            let t12 = t5 * t5;
            let t13 = f64x8::splat(1.0) / t12;
            let t15 = f64x8::splat(1.0) / t2 / t1;
            let t16 = t13 * t15;
            let t19 = t2 * t2;
            let t23 = -f64x8::splat(1.0) / t1 / f64x8::splat(3.0) + t8 / t19 / f64x8::splat(3.0);
            let t24 = f64x8::splat(1.0) / t8;
            let t25 = t23 * t24;
            let t26 = t25 * t3;
            let tvrho0 = -t7 - t11 + t1 * (-f64x8::splat(0.00066878) * t16 - f64x8::splat(0.0311) * t26);
            acc_vrho_0 = tvrho0;
            let tvrho1 = tvrho0;
            acc_vrho_1 = tvrho1;
            let t33 = f64x8::splat(1.0) / t12 / t5;
            let t34 = t1 * t1;
            let t36 = f64x8::splat(1.0) / t19 / t34;
            let t37 = t33 * t36;
            let t40 = f64x8::splat(1.0) / t2 / t34;
            let t41 = t13 * t40;
            let t45 = f64x8::splat(1.0) / t19 / t1;
            let t48 = f64x8::splat(2.0) / f64x8::splat(9.0) / t34 - f64x8::splat(2.0) / f64x8::splat(9.0) * t8 * t45;
            let t49 = t48 * t24;
            let t50 = t49 * t3;
            let t52 = t8 * t8;
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = t23 * t53;
            let t55 = t54 * t45;
            let t57 = t25 * t15;
            let tv2rho20 = -f64x8::splat(0.00133756) * t16 - f64x8::splat(0.0622) * t26 + t1 * (-f64x8::splat(2.5056957333333333e-05) * t37 + f64x8::splat(0.0008917066666666667) * t41 - f64x8::splat(0.0311) * t50 - f64x8::splat(0.010366666666666666) * t55 + f64x8::splat(0.010366666666666666) * t57);
            acc_v2rho2_0 = tv2rho20;
            let tv2rho21 = tv2rho20;
            acc_v2rho2_1 = tv2rho21;
            let tv2rho22 = tv2rho21;
            acc_v2rho2_2 = tv2rho22;
            let t66 = t12 * t12;
            let t67 = f64x8::splat(1.0) / t66;
            let t68 = t34 * t34;
            let t69 = f64x8::splat(1.0) / t68;
            let t70 = t67 * t69;
            let t72 = t34 * t1;
            let t74 = f64x8::splat(1.0) / t19 / t72;
            let t75 = t33 * t74;
            let t78 = f64x8::splat(1.0) / t2 / t72;
            let t79 = t13 * t78;
            let t81 = f64x8::splat(1.0) / t72;
            let t84 = f64x8::splat(10.0) / f64x8::splat(27.0) * t8 * t36 - f64x8::splat(10.0) / f64x8::splat(27.0) * t81;
            let t85 = t84 * t24;
            let t86 = t85 * t3;
            let t88 = t48 * t53;
            let t89 = t88 * t45;
            let t91 = t49 * t15;
            let t94 = f64x8::splat(1.0) / t52 / t8;
            let t95 = t23 * t94;
            let t96 = t95 * t81;
            let t98 = t54 * t36;
            let t100 = t25 * t40;
            let tv3rho30 = -f64x8::splat(7.5170872e-05) * t37 + f64x8::splat(0.00267512) * t41 - f64x8::splat(0.0933) * t50 - f64x8::splat(0.0311) * t55 + f64x8::splat(0.0311) * t57 + t1 * (-f64x8::splat(1.4082010021333333e-06) * t70 + f64x8::splat(0.00010022782933333333) * t75 - f64x8::splat(0.0020806488888888888) * t79 - f64x8::splat(0.0311) * t86 - f64x8::splat(0.020733333333333333) * t89 + f64x8::splat(0.020733333333333333) * t91 - f64x8::splat(0.006911111111111111) * t96 + f64x8::splat(0.020733333333333333) * t98 - f64x8::splat(0.013822222222222222) * t100);
            acc_v3rho3_0 = tv3rho30;
            let tv3rho31 = tv3rho30;
            acc_v3rho3_1 = tv3rho31;
            let tv3rho32 = tv3rho31;
            acc_v3rho3_2 = tv3rho32;
            let tv3rho33 = tv3rho32;
            acc_v3rho3_3 = tv3rho33;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        ip += 8;
    }
}
