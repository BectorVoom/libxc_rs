//! LDA_C_VWN_2 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_2.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_2 exc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_2_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = f64::sqrt(t11);
        let t15 = t12 + 1.86372 * t13 + 12.9352;
        let t16 = 1.0 / t15;
        let t20 = f64::ln(t4 * t10 * t16 / 4.0);
        let t21 = 0.0310907 * t20;
        let t22 = t13 + 3.72744;
        let t25 = f64::atan(6.15199081975908 / t22);
        let t26 = 0.038783294878113016 * t25;
        let t27 = t13 / 2.0;
        let t28 = t27 + 0.10498;
        let t29 = t28 * t28;
        let t31 = f64::ln(t29 * t16);
        let t32 = 0.0009690227711544374 * t31;
        let t33 = M_PI * M_PI;
        let t34 = 1.0 / t33;
        let t36 = t12 + 0.534175 * t13 + 11.4813;
        let t37 = 1.0 / t36;
        let t41 = f64::ln(t4 * t10 * t37 / 4.0);
        let t42 = t13 + 1.06835;
        let t45 = f64::atan(6.692072046645942 / t42);
        let t47 = t27 + 0.228344;
        let t48 = t47 * t47;
        let t50 = f64::ln(t48 * t37);
        let t53 = t34 * (t41 + 0.32323836906055065 * t45 + 0.021608710360898266 * t50);
        let t54 = rho0 - rho1;
        let t55 = 1.0 / t7;
        let t56 = t54 * t55;
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(zeta_threshold);
        let t60 = t59 * zeta_threshold;
        let t61 = pow_1_3(t57);
        let t63 = piecewise3(t58, t60, t61 * t57);
        let t64 = 1.0 - t56;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t60, t66 * t64);
        let t69 = t63 + t68 - 2.0;
        let t70 = t53 * t69;
        let t71 = M_CBRT2;
        let t72 = t71 - 1.0;
        let t74 = 1.0 / t72 / 2.0;
        let t75 = t54 * t54;
        let t76 = t75 * t75;
        let t77 = t7 * t7;
        let t78 = t77 * t77;
        let t79 = 1.0 / t78;
        let t82 = t74 * (-t76 * t79 + 1.0);
        let t83 = 9.0 * t72;
        let t84 = t82 * t83;
        let t86 = t70 * t84 / 24.0;
        let t88 = t12 + 10.06155 * t13 + 101.578;
        let t89 = 1.0 / t88;
        let t93 = f64::ln(t4 * t10 * t89 / 4.0);
        let t95 = t13 + 20.1231;
        let t98 = f64::atan(1.171685277708993 / t95);
        let t100 = t27 + 0.743294;
        let t101 = t100 * t100;
        let t103 = f64::ln(t101 * t89);
        let t106 = t12 + 6.536 * t13 + 42.7198;
        let t107 = 1.0 / t106;
        let t111 = f64::ln(t4 * t10 * t107 / 4.0);
        let t113 = t13 + 13.072;
        let t116 = f64::atan(0.0448998886412873 / t113);
        let t118 = t27 + 0.409286;
        let t119 = t118 * t118;
        let t121 = f64::ln(t119 * t107);
        let t123 = 0.01554535 * t93 + 0.6188180297906063 * t98 + 0.002667310007273315 * t103 - 0.0310907 * t111 - 20.521972937837504 * t116 - 0.004431373767749538 * t121;
        let t124 = t123 * t69;
        let t125 = t124 * t82;
        let t127 = t12 + 3.53021 * t13 + 18.0578;
        let t128 = 1.0 / t127;
        let t132 = f64::ln(t4 * t10 * t128 / 4.0);
        let t134 = t13 + 7.06042;
        let t137 = f64::atan(4.730926909560113 / t134);
        let t139 = t27 + 0.325;
        let t140 = t139 * t139;
        let t142 = f64::ln(t140 * t128);
        let t144 = 0.01554535 * t132 + 0.05249139316978094 * t137 + 0.0022478670955426118 * t142 - t21 - t26 - t32;
        let t146 = t144 * t69 * t74;
        let tzk0 = t21 + t26 + t32 - t86 - t125 + t146;
        zk[ip] += tzk0;
    }
}
