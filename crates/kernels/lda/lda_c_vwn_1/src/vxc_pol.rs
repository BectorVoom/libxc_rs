//! LDA_C_VWN_1 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_1.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_1 vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_vwn_1_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t22 = t13 + 3.72744;
        let t25 = f64::atan(6.15199081975908 / t22);
        let t27 = t13 / 2.0;
        let t28 = t27 + 0.10498;
        let t29 = t28 * t28;
        let t31 = f64::ln(t29 * t16);
        let t33 = 0.0310907 * t20 + 0.038783294878113016 * t25 + 0.0009690227711544374 * t31;
        let t34 = rho0 - rho1;
        let t35 = 1.0 / t7;
        let t36 = t34 * t35;
        let t37 = 1.0 + t36;
        let t38 = t37 <= zeta_threshold;
        let t39 = pow_1_3(zeta_threshold);
        let t40 = t39 * zeta_threshold;
        let t41 = pow_1_3(t37);
        let t43 = piecewise3(t38, t40, t41 * t37);
        let t44 = 1.0 - t36;
        let t45 = t44 <= zeta_threshold;
        let t46 = pow_1_3(t44);
        let t48 = piecewise3(t45, t40, t46 * t44);
        let t49 = t43 + t48 - 2.0;
        let t50 = M_CBRT2;
        let t53 = 1.0 / (2.0 * t50 - 2.0);
        let t55 = -t49 * t53 + 1.0;
        let t56 = t33 * t55;
        let t58 = t12 + 3.53021 * t13 + 18.0578;
        let t59 = 1.0 / t58;
        let t63 = f64::ln(t4 * t10 * t59 / 4.0);
        let t65 = t13 + 7.06042;
        let t68 = f64::atan(4.730926909560113 / t65);
        let t70 = t27 + 0.325;
        let t71 = t70 * t70;
        let t73 = f64::ln(t71 * t59);
        let t75 = 0.01554535 * t63 + 0.05249139316978094 * t68 + 0.0022478670955426118 * t73;
        let t77 = t75 * t49 * t53;
        let tzk0 = t56 + t77;
        zk[ip] += tzk0;
        let t79 = 1.0 / t8 / t7;
        let t80 = t6 * t79;
        let t84 = t4 * t6;
        let t85 = t15 * t15;
        let t86 = 1.0 / t85;
        let t87 = t9 * t86;
        let t88 = t4 * t80;
        let t89 = t88 / 12.0;
        let t90 = 1.0 / t13;
        let t91 = t90 * t1;
        let t92 = t3 * t6;
        let t94 = t91 * t92 * t79;
        let t96 = -t89 - 0.31062 * t94;
        let t101 = t1 * t1;
        let t103 = 1.0 / t3;
        let t104 = (-t4 * t80 * t16 / 12.0 - t84 * t87 * t96 / 4.0) * t101 * t103;
        let t105 = t5 * t8;
        let t106 = t105 * t15;
        let t109 = t22 * t22;
        let t110 = 1.0 / t109;
        let t112 = t110 * t90 * t1;
        let t114 = 37.8469910464 * t110 + 1.0;
        let t115 = 1.0 / t114;
        let t120 = t28 * t16;
        let t121 = t120 * t90;
        let t124 = t29 * t86;
        let t126 = -t121 * t88 / 6.0 - t124 * t96;
        let t127 = 1.0 / t29;
        let t128 = t126 * t127;
        let t131 = 0.010363566666666667 * t104 * t106 + 0.03976574567502677 * t112 * t92 * t79 * t115 + 0.0009690227711544374 * t128 * t15;
        let t132 = t131 * t55;
        let t133 = t7 * t7;
        let t134 = 1.0 / t133;
        let t135 = t34 * t134;
        let t136 = t35 - t135;
        let t139 = piecewise3(t38, 0.0, 4.0 / 3.0 * t41 * t136);
        let t140 = -t136;
        let t143 = piecewise3(t45, 0.0, 4.0 / 3.0 * t46 * t140);
        let t144 = t139 + t143;
        let t146 = t33 * t144 * t53;
        let t150 = t58 * t58;
        let t151 = 1.0 / t150;
        let t152 = t9 * t151;
        let t154 = -t89 - 0.5883683333333334 * t94;
        let t160 = (-t4 * t80 * t59 / 12.0 - t84 * t152 * t154 / 4.0) * t101 * t103;
        let t161 = t105 * t58;
        let t164 = t65 * t65;
        let t165 = 1.0 / t164;
        let t167 = t165 * t90 * t1;
        let t169 = 22.3816694236 * t165 + 1.0;
        let t170 = 1.0 / t169;
        let t175 = t70 * t59;
        let t176 = t175 * t90;
        let t179 = t71 * t151;
        let t181 = -t176 * t88 / 6.0 - t179 * t154;
        let t182 = 1.0 / t71;
        let t183 = t181 * t182;
        let t186 = 0.005181783333333334 * t160 * t161 + 0.041388824077869424 * t167 * t92 * t79 * t170 + 0.0022478670955426118 * t183 * t58;
        let t188 = t186 * t49 * t53;
        let t190 = t75 * t144 * t53;
        let tvrho0 = t56 + t77 + t7 * (t132 - t146 + t188 + t190);
        vrho[ip * 2] += tvrho0;
        let t193 = -t35 - t135;
        let t196 = piecewise3(t38, 0.0, 4.0 / 3.0 * t41 * t193);
        let t197 = -t193;
        let t200 = piecewise3(t45, 0.0, 4.0 / 3.0 * t46 * t197);
        let t201 = t196 + t200;
        let t203 = t33 * t201 * t53;
        let t205 = t75 * t201 * t53;
        let tvrho1 = t56 + t77 + t7 * (t132 - t203 + t188 + t205);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
