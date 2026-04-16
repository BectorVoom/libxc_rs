//! GGA_C_REGTPSS exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_regtpss.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_regtpss_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t11 = t4 * t6 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = rho0 - rho1;
        let t35 = t34 * t34;
        let t36 = t35 * t35;
        let t37 = t7 * t7;
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t36 * t39;
        let t41 = 1.0 / t7;
        let t42 = t34 * t41;
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(zeta_threshold);
        let t46 = t45 * zeta_threshold;
        let t47 = pow_1_3(t43);
        let t48 = t47 * t43;
        let t49 = piecewise3(t44, t46, t48);
        let t50 = 1.0 - t42;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t53 = t52 * t50;
        let t54 = piecewise3(t51, t46, t53);
        let t55 = t49 + t54 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t55 * t59;
        let t62 = 1.0 + 0.5137e-1 * t11;
        let t67 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
        let t70 = 1.0 + 0.32163958997385070134e2 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.278125e-1 * t11;
        let t80 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t83 = 1.0 + 0.29608749977793437516e2 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.310907e-1 * t62 * t71 + t33 - 0.19751673498613801407e-1 * t85;
        let t88 = t60 * t87;
        let t89 = t40 * t88;
        let t91 = 0.19751673498613801407e-1 * t60 * t85;
        let t92 = f64::ln(2.0);
        let t93 = 1.0 - t92;
        let t94 = M_PI * M_PI;
        let t95 = 1.0 / t94;
        let t96 = t93 * t95;
        let t97 = t45 * t45;
        let t98 = t47 * t47;
        let t99 = piecewise3(t44, t97, t98);
        let t100 = t52 * t52;
        let t101 = piecewise3(t51, t97, t100);
        let t103 = t99 / 2.0 + t101 / 2.0;
        let t104 = t103 * t103;
        let t105 = t104 * t103;
        let t107 = 1.0 + 0.25e-1 * t11;
        let t109 = 1.0 + 0.4445e-1 * t11;
        let t110 = 1.0 / t109;
        let t111 = t107 * t110;
        let t113 = sigma0 + 2.0 * sigma1 + sigma2;
        let t115 = 1.0 / t8 / t37;
        let t116 = t113 * t115;
        let t118 = 1.0 / t104;
        let t120 = 1.0 / t3;
        let t121 = t120 * t5;
        let t122 = t118 * t19 * t121;
        let t125 = 1.0 / t93;
        let t127 = (-t33 + t89 + t91) * t125;
        let t128 = 1.0 / t105;
        let t129 = t94 * t128;
        let t131 = f64::exp(-t127 * t129);
        let t132 = t131 - 1.0;
        let t133 = 1.0 / t132;
        let t134 = t125 * t133;
        let t135 = t113 * t113;
        let t136 = t134 * t135;
        let t137 = t111 * t136;
        let t139 = 1.0 / t22 / t38;
        let t140 = t56 * t56;
        let t141 = t139 * t140;
        let t142 = t104 * t104;
        let t143 = 1.0 / t142;
        let t144 = t141 * t143;
        let t145 = 1.0 / t20;
        let t146 = t1 * t145;
        let t147 = t146 * t6;
        let t148 = t144 * t147;
        let t151 = t116 * t56 * t122 / 96.0 + 0.21437009059034868486e-3 * t137 * t148;
        let t152 = t151 * t125;
        let t153 = t134 * t151;
        let t156 = 1.0 + 0.65854491829355115987e0 * t111 * t153;
        let t157 = 1.0 / t156;
        let t158 = t152 * t157;
        let t161 = 1.0 + 0.65854491829355115987e0 * t111 * t158;
        let t162 = f64::ln(t161);
        let t164 = t96 * t105 * t162;
        let tzk0 = -t33 + t89 + t91 + t164;
        zk[ip] += tzk0;
    }
}
