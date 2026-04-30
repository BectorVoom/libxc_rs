//! LDA_C_VWN_3 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 98 shared lines across all orders.
//! Delta: 98 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_3 exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_vwn_3_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (98 lines) ---
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
        let t34 = t12 + 3.53021 * t13 + 18.0578;
        let t35 = 1.0 / t34;
        let t39 = f64::ln(t4 * t10 * t35 / 4.0);
        let t41 = t13 + 7.06042;
        let t44 = f64::atan(4.730926909560113 / t41);
        let t46 = t27 + 0.325;
        let t47 = t46 * t46;
        let t49 = f64::ln(t47 * t35);
        let t51 = 0.01554535 * t39 + 0.05249139316978094 * t44 + 0.0022478670955426118 * t49 - t21 - t26 - t32;
        let t53 = t12 + 10.06155 * t13 + 101.578;
        let t54 = 1.0 / t53;
        let t58 = f64::ln(t4 * t10 * t54 / 4.0);
        let t60 = t13 + 20.1231;
        let t63 = f64::atan(1.171685277708993 / t60);
        let t65 = t27 + 0.743294;
        let t66 = t65 * t65;
        let t68 = f64::ln(t66 * t54);
        let t71 = t12 + 6.536 * t13 + 42.7198;
        let t72 = 1.0 / t71;
        let t76 = f64::ln(t4 * t10 * t72 / 4.0);
        let t78 = t13 + 13.072;
        let t81 = f64::atan(0.0448998886412873 / t78);
        let t83 = t27 + 0.409286;
        let t84 = t83 * t83;
        let t86 = f64::ln(t84 * t72);
        let t88 = 0.01554535 * t58 + 0.6188180297906063 * t63 + 0.002667310007273315 * t68 - 0.0310907 * t76 - 20.521972937837504 * t81 - 0.004431373767749538 * t86;
        let t89 = 1.0 / t88;
        let t90 = t51 * t89;
        let t91 = M_PI * M_PI;
        let t92 = 1.0 / t91;
        let t94 = t12 + 0.534175 * t13 + 11.4813;
        let t95 = 1.0 / t94;
        let t99 = f64::ln(t4 * t10 * t95 / 4.0);
        let t100 = t13 + 1.06835;
        let t103 = f64::atan(6.692072046645942 / t100);
        let t105 = t27 + 0.228344;
        let t106 = t105 * t105;
        let t108 = f64::ln(t106 * t95);
        let t111 = t92 * (t99 + 0.32323836906055065 * t103 + 0.021608710360898266 * t108);
        let t112 = t90 * t111;
        let t113 = rho0 - rho1;
        let t114 = 1.0 / t7;
        let t115 = t113 * t114;
        let t116 = 1.0 + t115;
        let t117 = t116 <= zeta_threshold;
        let t118 = pow_1_3(zeta_threshold);
        let t119 = t118 * zeta_threshold;
        let t120 = pow_1_3(t116);
        let t122 = piecewise3(t117, t119, t120 * t116);
        let t123 = 1.0 - t115;
        let t124 = t123 <= zeta_threshold;
        let t125 = pow_1_3(t123);
        let t127 = piecewise3(t124, t119, t125 * t123);
        let t128 = t122 + t127 - 2.0;
        let t129 = M_CBRT2;
        let t130 = t129 - 1.0;
        let t132 = 1.0 / t130 / 2.0;
        let t133 = t128 * t132;
        let t134 = t113 * t113;
        let t135 = t134 * t134;
        let t136 = t7 * t7;
        let t137 = t136 * t136;
        let t138 = 1.0 / t137;
        let t140 = -t135 * t138 + 1.0;
        let t141 = 9.0 * t130;
        let t142 = t140 * t141;
        let t143 = t133 * t142;
        let t145 = t112 * t143 / 24.0;
        let t146 = t51 * t128;
        let t147 = t132 * t135;
        let t148 = t147 * t138;
        let t149 = t146 * t148;
        let tzk0 = t21 + t26 + t32 - t145 + t149;
        zk[ip] += tzk0;
    }
}
