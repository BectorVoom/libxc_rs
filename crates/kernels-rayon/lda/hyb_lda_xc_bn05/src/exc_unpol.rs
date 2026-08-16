//! HYB_LDA_XC_BN05 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/hyb_lda_xc_bn05.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

/// HYB_LDA_XC_BN05 exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn hyb_lda_xc_bn05_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t3 * t1;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t6 * t4;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = 1.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t13 = piecewise3(t10, t11 * zeta_threshold, 1.0);
        let t14 = t13 * t9;
        let t15 = pow_1_3(rho[ip]);
        let t16 = pow_1_3(9.0);
        let t17 = t16 * t16;
        let t18 = t3 * t3;
        let t20 = param_hyb_omega_0 * t18 * t17;
        let t21 = 1.0 / t15;
        let t23 = piecewise3(t10, t11, 1.0);
        let t24 = 1.0 / t23;
        let t27 = t24 * t21 * t1 * t20 / 18.0;
        let t28 = 1.92 <= t27;
        let t29 = 1.92 < t27;
        let t30 = piecewise3(t29, t27, 1.92);
        let t31 = t30 * t30;
        let t34 = t31 * t31;
        let t35 = 1.0 / t34;
        let t37 = t34 * t31;
        let t38 = 1.0 / t37;
        let t40 = t34 * t34;
        let t41 = 1.0 / t40;
        let t43 = t40 * t31;
        let t44 = 1.0 / t43;
        let t46 = t40 * t34;
        let t47 = 1.0 / t46;
        let t49 = t40 * t37;
        let t50 = 1.0 / t49;
        let t52 = t40 * t40;
        let t53 = 1.0 / t52;
        let t56 = 1.0 / t52 / t31;
        let t59 = 1.0 / t52 / t34;
        let t62 = 1.0 / t52 / t37;
        let t65 = 1.0 / t52 / t40;
        let t68 = 1.0 / t52 / t43;
        let t71 = 1.0 / t52 / t46;
        let t74 = 1.0 / t52 / t49;
        let t76 = t52 * t52;
        let t77 = 1.0 / t76;
        let t80 = 1.0 / t76 / t31;
        let t83 = 1.0 / t76 / t34;
        let t85 = 1.0 / t31 / 9.0 - t35 / 30.0 + t38 / 70.0 - t41 / 135.0 + t44 / 231.0 - t47 / 364.0 + t50 / 540.0 - t53 / 765.0 + t56 / 1045.0 - t59 / 1386.0 + t62 / 1794.0 - t65 / 2275.0 + t68 / 2835.0 - t71 / 3480.0 + t74 / 4216.0 - t77 / 5049.0 + t80 / 5985.0 - t83 / 7030.0;
        let t86 = piecewise3(t29, 1.92, t27);
        let t87 = f64::atan2(1.0, t86);
        let t88 = t86 * t86;
        let t89 = t88 + 3.0;
        let t90 = 1.0 / t88;
        let t91 = 1.0 + t90;
        let t92 = f64::ln(t91);
        let t94 = -t92 * t89 + 1.0;
        let t97 = t87 + t94 * t86 / 4.0;
        let t101 = piecewise3(t28, t85, 1.0 - 8.0 / 3.0 * t97 * t86);
        let t105 = 3.0 / 16.0 * t101 * t15 * t14 * t7;
        let t107 = t21 * t6 * t4;
        let t109 = 1.0 + 0.053425 * t107;
        let t110 = f64::sqrt(t107);
        let t113 = pow_3_2(t107);
        let t115 = t1 * t1;
        let t116 = t18 * t115;
        let t117 = t15 * t15;
        let t118 = 1.0 / t117;
        let t120 = t118 * t5 * t116;
        let t122 = 3.79785 * t110 + 0.8969 * t107 + 0.204775 * t113 + 0.123235 * t120;
        let t125 = 1.0 + 16.081979498692537 / t122;
        let t126 = f64::ln(t125);
        let t134 = 1.0 / (2.0 * t8 - 2.0) * (2.0 * t13 - 2.0);
        let t136 = 1.0 + 0.0278125 * t107;
        let t141 = 5.1785 * t110 + 0.905775 * t107 + 0.1100325 * t113 + 0.1241775 * t120;
        let t144 = 1.0 + 29.608749977793437 / t141;
        let t145 = f64::ln(t144);
        let t149 = -0.0621814 * t126 * t109 + 0.0197516734986138 * t145 * t136 * t134;
        let t152 = 3.2 - 0.225 * t107 + t120 / 4.0;
        let t153 = 1.0 / t152;
        let t155 = 3.4602 * t153 * t149;
        let tzk0 = -t105 + t155;
        zk[ip] += tzk0;
    }
}
