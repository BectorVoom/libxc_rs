//! LDA_C_GK72 lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gk72.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gk72_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
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
        let mut acc_v4rho4_0 = V_ZERO;
        let mut acc_v4rho4_1 = V_ZERO;
        let mut acc_v4rho4_2 = V_ZERO;
        let mut acc_v4rho4_3 = V_ZERO;
        let mut acc_v4rho4_4 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t10 = t6 / t8;
            let t11 = t4 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = (t12).simd_lt(f64x8::splat(0.7));
            let t14 = (simd::ln(t12));
            let t21 = (t12).simd_lt(f64x8::splat(10.0));
            let t24 = t1 * t1;
            let t26 = t24 / t3;
            let t30 = ((f64x8::splat(4.0)).sqrt());
            let t31 = ((t11).sqrt());
            let t36 = t3 * t3;
            let t38 = t1 / t36;
            let t39 = t8 * t8;
            let t43 = t24 * t36;
            let t45 = t5 / t39;
            let t49 = f64x8::splat(1.0) / t31 / t43 / t45 / f64x8::splat(4.0);
            let tzk0 = ((t13).select(f64x8::splat(0.0311) * t14 - f64x8::splat(0.048) + f64x8::splat(0.00225) * t4 * t10 * t14 - f64x8::splat(0.00425) * t11, (t21).select(-f64x8::splat(0.06156) + f64x8::splat(0.01898) * t14, f64x8::splat(0.146) * t26 * t5 * t8 + f64x8::splat(5.3) * t30 / t31 / t11 - f64x8::splat(0.49) * t38 * t6 * t39 - f64x8::splat(6.4) * t30 * t49)));
            acc_zk = tzk0;
            let t53 = f64x8::splat(1.0) / t7;
            let t56 = f64x8::splat(1.0) / t8 / t7;
            let t57 = t6 * t56;
            let t67 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t68 = t67 * t49;
            let t69 = t4 * t56;
            let t77 = f64x8::splat(1.0) / t31 / t2 / t53 / f64x8::splat(48.0);
            let t78 = t67 * t77;
            let t82 = ((t13).select(-f64x8::splat(0.010366666666666666) * t53 - f64x8::splat(0.00075) * t4 * t57 * t14 + f64x8::splat(0.0006666666666666666) * t4 * t57, (t21).select(-f64x8::splat(0.006326666666666667) * t53, f64x8::splat(0.048666666666666664) * t26 * t45 + f64x8::splat(10.6) * t68 * t69 - f64x8::splat(0.32666666666666666) * t38 * t10 - f64x8::splat(21.333333333333332) * t78 * t69)));
            let tvrho0 = t7 * t82 + tzk0;
            acc_vrho_0 = tvrho0;
            let tvrho1 = tvrho0;
            acc_vrho_1 = tvrho1;
            let t85 = t7 * t7;
            let t86 = f64x8::splat(1.0) / t85;
            let t89 = f64x8::splat(1.0) / t8 / t85;
            let t90 = t6 * t89;
            let t100 = t5 / t39 / t7;
            let t103 = t67 * t67;
            let t104 = t103 * t103;
            let t105 = t104 * t67;
            let t106 = t105 * t77;
            let t108 = f64x8::splat(1.0) / t39 / t85;
            let t109 = t43 * t108;
            let t112 = t4 * t89;
            let t122 = f64x8::splat(1.0) / t31 / t1 / t3 / t2 / t57 / f64x8::splat(48.0);
            let t123 = t105 * t122;
            let t129 = ((t13).select(f64x8::splat(0.010366666666666666) * t86 + f64x8::splat(0.001) * t4 * t90 * t14 - f64x8::splat(0.0006388888888888889) * t4 * t90, (t21).select(f64x8::splat(0.006326666666666667) * t86, -f64x8::splat(0.03244444444444444) * t26 * t100 + f64x8::splat(8.833333333333334) * t106 * t109 - f64x8::splat(14.133333333333333) * t68 * t112 + f64x8::splat(0.10888888888888888) * t38 * t57 - f64x8::splat(24.88888888888889) * t123 * t109 + f64x8::splat(28.444444444444443) * t78 * t112)));
            let tv2rho20 = t7 * t129 + f64x8::splat(2.0) * t82;
            acc_v2rho2_0 = tv2rho20;
            let tv2rho21 = tv2rho20;
            acc_v2rho2_1 = tv2rho21;
            let tv2rho22 = tv2rho21;
            acc_v2rho2_2 = tv2rho22;
            let t132 = t85 * t7;
            let t133 = f64x8::splat(1.0) / t132;
            let t136 = f64x8::splat(1.0) / t8 / t132;
            let t137 = t6 * t136;
            let t148 = t30 * t122;
            let t149 = t85 * t85;
            let t150 = f64x8::splat(1.0) / t149;
            let t151 = t2 * t150;
            let t155 = f64x8::splat(1.0) / t39 / t132;
            let t156 = t43 * t155;
            let t159 = t4 * t136;
            let t169 = f64x8::splat(1.0) / t31 / t24 / t36 / t2 / t100 / f64x8::splat(192.0);
            let t170 = t30 * t169;
            let t178 = ((t13).select(-f64x8::splat(0.020733333333333333) * t133 - f64x8::splat(0.0023333333333333335) * t4 * t137 * t14 + f64x8::splat(0.0011574074074074073) * t4 * t137, (t21).select(-f64x8::splat(0.012653333333333334) * t133, f64x8::splat(0.05407407407407407) * t26 * t5 * t108 + f64x8::splat(123.66666666666667) * t148 * t151 - f64x8::splat(35.333333333333336) * t106 * t156 + f64x8::splat(32.977777777777774) * t68 * t159 - f64x8::splat(0.1451851851851852) * t38 * t90 - f64x8::splat(448.0) * t170 * t151 + f64x8::splat(99.55555555555556) * t123 * t156 - f64x8::splat(66.37037037037037) * t78 * t159)));
            let tv3rho30 = t7 * t178 + f64x8::splat(3.0) * t129;
            acc_v3rho3_0 = tv3rho30;
            let tv3rho31 = tv3rho30;
            acc_v3rho3_1 = tv3rho31;
            let tv3rho32 = tv3rho31;
            acc_v3rho3_2 = tv3rho32;
            let tv3rho33 = tv3rho32;
            acc_v3rho3_3 = tv3rho33;
            let t183 = f64x8::splat(1.0) / t8 / t149;
            let t184 = t6 * t183;
            let t197 = t149 * t7;
            let t201 = f64x8::splat(1.0) / t8 / t197 * t1 * t3;
            let t205 = t2 / t197;
            let t210 = t43 / t39 / t149;
            let t213 = t4 * t183;
            let t218 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t235 = ((t13).select(f64x8::splat(0.0622) * t150 + f64x8::splat(0.0077777777777777776) * t4 * t184 * t14 - f64x8::splat(0.003080246913580247) * t4 * t184, (t21).select(f64x8::splat(0.03796) * t150, -f64x8::splat(0.14419753086419754) * t26 * t5 * t155 + f64x8::splat(742.0) * t67 * t169 * t2 * t201 - f64x8::splat(989.3333333333334) * t148 * t205 + f64x8::splat(157.03703703703704) * t106 * t210 - f64x8::splat(109.92592592592592) * t68 * t213 + f64x8::splat(0.3387654320987654) * t38 * t137 - f64x8::splat(1.4259259259259258) * t67 / t31 * t218 / t86 * t2 * t201 + f64x8::splat(3584.0) * t170 * t205 - f64x8::splat(442.4691358024691) * t123 * t210 + f64x8::splat(221.23456790123456) * t78 * t213)));
            let tv4rho40 = t7 * t235 + f64x8::splat(4.0) * t178;
            acc_v4rho4_0 = tv4rho40;
            let tv4rho41 = tv4rho40;
            acc_v4rho4_1 = tv4rho41;
            let tv4rho42 = tv4rho41;
            acc_v4rho4_2 = tv4rho42;
            let tv4rho43 = tv4rho42;
            acc_v4rho4_3 = tv4rho43;
            let tv4rho44 = tv4rho43;
            acc_v4rho4_4 = tv4rho44;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v4rho4, ip, m, 5, 0, acc_v4rho4_0);
        store_strided(v4rho4, ip, m, 5, 1, acc_v4rho4_1);
        store_strided(v4rho4, ip, m, 5, 2, acc_v4rho4_2);
        store_strided(v4rho4, ip, m, 5, 3, acc_v4rho4_3);
        store_strided(v4rho4, ip, m, 5, 4, acc_v4rho4_4);
        ip += 8;
    }
}
