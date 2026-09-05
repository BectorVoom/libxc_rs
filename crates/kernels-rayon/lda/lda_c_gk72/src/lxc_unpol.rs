//! LDA_C_GK72 lxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gk72_lxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t9 = t6 / t7;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = (t11).simd_lt(f64x8::splat(0.7));
            let t13 = (simd::ln(t11));
            let t20 = (t11).simd_lt(f64x8::splat(10.0));
            let t23 = t1 * t1;
            let t25 = t23 / t3;
            let t29 = ((f64x8::splat(4.0)).sqrt());
            let t30 = ((t10).sqrt());
            let t35 = t3 * t3;
            let t37 = t1 / t35;
            let t38 = t7 * t7;
            let t42 = t23 * t35;
            let t44 = t5 / t38;
            let t48 = f64x8::splat(1.0) / t30 / t42 / t44 / f64x8::splat(4.0);
            let tzk0 = ((t12).select(f64x8::splat(0.0311) * t13 - f64x8::splat(0.048) + f64x8::splat(0.00225) * t4 * t9 * t13 - f64x8::splat(0.00425) * t10, (t20).select(-f64x8::splat(0.06156) + f64x8::splat(0.01898) * t13, f64x8::splat(0.146) * t25 * t5 * t7 + f64x8::splat(5.3) * t29 / t30 / t10 - f64x8::splat(0.49) * t37 * t6 * t38 - f64x8::splat(6.4) * t29 * t48)));
            acc_zk = tzk0;
            let t52 = f64x8::splat(1.0) / v_rho;
            let t55 = f64x8::splat(1.0) / t7 / v_rho;
            let t56 = t6 * t55;
            let t66 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t67 = t66 * t48;
            let t68 = t4 * t55;
            let t76 = f64x8::splat(1.0) / t30 / t2 / t52 / f64x8::splat(48.0);
            let t77 = t66 * t76;
            let t81 = ((t12).select(-f64x8::splat(0.010366666666666666) * t52 - f64x8::splat(0.00075) * t4 * t56 * t13 + f64x8::splat(0.0006666666666666666) * t4 * t56, (t20).select(-f64x8::splat(0.006326666666666667) * t52, f64x8::splat(0.048666666666666664) * t25 * t44 + f64x8::splat(10.6) * t67 * t68 - f64x8::splat(0.32666666666666666) * t37 * t9 - f64x8::splat(21.333333333333332) * t77 * t68)));
            let tvrho0 = v_rho * t81 + tzk0;
            acc_vrho = tvrho0;
            let t84 = v_rho * v_rho;
            let t85 = f64x8::splat(1.0) / t84;
            let t88 = f64x8::splat(1.0) / t7 / t84;
            let t89 = t6 * t88;
            let t99 = t5 / t38 / v_rho;
            let t102 = t66 * t66;
            let t103 = t102 * t102;
            let t104 = t103 * t66;
            let t105 = t104 * t76;
            let t107 = f64x8::splat(1.0) / t38 / t84;
            let t108 = t42 * t107;
            let t111 = t4 * t88;
            let t121 = f64x8::splat(1.0) / t30 / t1 / t3 / t2 / t56 / f64x8::splat(48.0);
            let t122 = t104 * t121;
            let t128 = ((t12).select(f64x8::splat(0.010366666666666666) * t85 + f64x8::splat(0.001) * t4 * t89 * t13 - f64x8::splat(0.0006388888888888889) * t4 * t89, (t20).select(f64x8::splat(0.006326666666666667) * t85, -f64x8::splat(0.03244444444444444) * t25 * t99 + f64x8::splat(8.833333333333334) * t105 * t108 - f64x8::splat(14.133333333333333) * t67 * t111 + f64x8::splat(0.10888888888888888) * t37 * t56 - f64x8::splat(24.88888888888889) * t122 * t108 + f64x8::splat(28.444444444444443) * t77 * t111)));
            let tv2rho20 = v_rho * t128 + f64x8::splat(2.0) * t81;
            acc_v2rho2 = tv2rho20;
            let t131 = t84 * v_rho;
            let t132 = f64x8::splat(1.0) / t131;
            let t135 = f64x8::splat(1.0) / t7 / t131;
            let t136 = t6 * t135;
            let t147 = t29 * t121;
            let t148 = t84 * t84;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t2 * t149;
            let t154 = f64x8::splat(1.0) / t38 / t131;
            let t155 = t42 * t154;
            let t158 = t4 * t135;
            let t168 = f64x8::splat(1.0) / t30 / t23 / t35 / t2 / t99 / f64x8::splat(192.0);
            let t169 = t29 * t168;
            let t177 = ((t12).select(-f64x8::splat(0.020733333333333333) * t132 - f64x8::splat(0.0023333333333333335) * t4 * t136 * t13 + f64x8::splat(0.0011574074074074073) * t4 * t136, (t20).select(-f64x8::splat(0.012653333333333334) * t132, f64x8::splat(0.05407407407407407) * t25 * t5 * t107 + f64x8::splat(123.66666666666667) * t147 * t150 - f64x8::splat(35.333333333333336) * t105 * t155 + f64x8::splat(32.977777777777774) * t67 * t158 - f64x8::splat(0.1451851851851852) * t37 * t89 - f64x8::splat(448.0) * t169 * t150 + f64x8::splat(99.55555555555556) * t122 * t155 - f64x8::splat(66.37037037037037) * t77 * t158)));
            let tv3rho30 = v_rho * t177 + f64x8::splat(3.0) * t128;
            acc_v3rho3 = tv3rho30;
            let t182 = f64x8::splat(1.0) / t7 / t148;
            let t183 = t6 * t182;
            let t196 = t148 * v_rho;
            let t200 = f64x8::splat(1.0) / t7 / t196 * t1 * t3;
            let t204 = t2 / t196;
            let t209 = t42 / t38 / t148;
            let t212 = t4 * t182;
            let t217 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t234 = ((t12).select(f64x8::splat(0.0622) * t149 + f64x8::splat(0.0077777777777777776) * t4 * t183 * t13 - f64x8::splat(0.003080246913580247) * t4 * t183, (t20).select(f64x8::splat(0.03796) * t149, -f64x8::splat(0.14419753086419754) * t25 * t5 * t154 + f64x8::splat(742.0) * t66 * t168 * t2 * t200 - f64x8::splat(989.3333333333334) * t147 * t204 + f64x8::splat(157.03703703703704) * t105 * t209 - f64x8::splat(109.92592592592592) * t67 * t212 + f64x8::splat(0.3387654320987654) * t37 * t136 - f64x8::splat(1.4259259259259258) * t66 / t30 * t217 / t85 * t2 * t200 + f64x8::splat(3584.0) * t169 * t204 - f64x8::splat(442.4691358024691) * t122 * t209 + f64x8::splat(221.23456790123456) * t77 * t212)));
            let tv4rho40 = v_rho * t234 + f64x8::splat(4.0) * t177;
            acc_v4rho4 = tv4rho40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
