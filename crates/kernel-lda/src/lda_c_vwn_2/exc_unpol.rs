//! LDA_C_VWN_2 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 69 shared lines across all orders.
//! Delta: 69 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_2 exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_vwn_2_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (69 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = f64::sqrt(t10);
        let t14 = t11 + 1.86372 * t12 + 12.9352;
        let t15 = 1.0 / t14;
        let t19 = f64::ln(t4 * t9 * t15 / 4.0);
        let t20 = 0.0310907 * t19;
        let t21 = t12 + 3.72744;
        let t24 = f64::atan(6.15199081975908 / t21);
        let t25 = 0.038783294878113016 * t24;
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498;
        let t28 = t27 * t27;
        let t30 = f64::ln(t28 * t15);
        let t31 = 0.0009690227711544374 * t30;
        let t32 = M_PI * M_PI;
        let t33 = 1.0 / t32;
        let t35 = t11 + 0.534175 * t12 + 11.4813;
        let t36 = 1.0 / t35;
        let t40 = f64::ln(t4 * t9 * t36 / 4.0);
        let t41 = t12 + 1.06835;
        let t44 = f64::atan(6.692072046645942 / t41);
        let t46 = t26 + 0.228344;
        let t47 = t46 * t46;
        let t49 = f64::ln(t47 * t36);
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(1.0 <= zeta_threshold, t54 * zeta_threshold, 1.0);
        let t58 = 2.0 * t56 - 2.0;
        let t59 = M_CBRT2;
        let t60 = t59 - 1.0;
        let t62 = 1.0 / t60 / 2.0;
        let t65 = 9.0 * t58 * t62 * t60;
        let t67 = t33 * (t40 + 0.32323836906055065 * t44 + 0.021608710360898266 * t49) * t65 / 24.0;
        let t69 = t11 + 10.06155 * t12 + 101.578;
        let t70 = 1.0 / t69;
        let t74 = f64::ln(t4 * t9 * t70 / 4.0);
        let t76 = t12 + 20.1231;
        let t79 = f64::atan(1.171685277708993 / t76);
        let t81 = t26 + 0.743294;
        let t82 = t81 * t81;
        let t84 = f64::ln(t82 * t70);
        let t87 = t11 + 6.536 * t12 + 42.7198;
        let t88 = 1.0 / t87;
        let t92 = f64::ln(t4 * t9 * t88 / 4.0);
        let t94 = t12 + 13.072;
        let t97 = f64::atan(0.0448998886412873 / t94);
        let t99 = t26 + 0.409286;
        let t100 = t99 * t99;
        let t102 = f64::ln(t100 * t88);
        let t106 = (0.01554535 * t74 + 0.6188180297906063 * t79 + 0.002667310007273315 * t84 - 0.0310907 * t92 - 20.521972937837504 * t97 - 0.004431373767749538 * t102) * t58 * t62;
        let t108 = t11 + 3.53021 * t12 + 18.0578;
        let t109 = 1.0 / t108;
        let t113 = f64::ln(t4 * t9 * t109 / 4.0);
        let t115 = t12 + 7.06042;
        let t118 = f64::atan(4.730926909560113 / t115);
        let t120 = t26 + 0.325;
        let t121 = t120 * t120;
        let t123 = f64::ln(t121 * t109);
        let t127 = (0.01554535 * t113 + 0.05249139316978094 * t118 + 0.0022478670955426118 * t123 - t20 - t25 - t31) * t58 * t62;
        let tzk0 = t20 + t25 + t31 - t67 - t106 + t127;
        zk[ip] += tzk0;
    }
}
