//! GGA_C_BMK exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_bmk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_bmk_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_c_ab_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = piecewise3::<f64>(t3, zeta_threshold, 1.0);
        let t6 = M_CBRT3;
        let t7 = 1.0 / M_PI;
        let t8 = pow_1_3::<f64>(t7);
        let t9 = t6 * t8;
        let t10 = M_CBRT4;
        let t11 = t10 * t10;
        let t12 = t9 * t11;
        let t13 = pow_1_3::<f64>(rho[ip]);
        let t14 = 1.0 / t13;
        let t15 = M_CBRT2;
        let t17 = pow_1_3::<f64>(zeta_threshold);
        let t19 = piecewise3::<f64>(t3, 1.0 / t17, 1.0);
        let t21 = t12 * t14 * t15 * t19;
        let t23 = 1.0 + 0.53425e-1 * t21;
        let t24 = f64::sqrt(t21);
        let t27 = pow_3_2::<f64>(t21);
        let t29 = t6 * t6;
        let t30 = t8 * t8;
        let t31 = t29 * t30;
        let t32 = t31 * t10;
        let t33 = t13 * t13;
        let t34 = 1.0 / t33;
        let t35 = t15 * t15;
        let t37 = t19 * t19;
        let t39 = t32 * t34 * t35 * t37;
        let t41 = 0.379785e1 * t24 + 0.8969e0 * t21 + 0.204775e0 * t27 + 0.123235e0 * t39;
        let t44 = 1.0 + 0.16081824322151104822e2 / t41;
        let t45 = f64::ln(t44);
        let t47 = 0.62182e-1 * t23 * t45;
        let t49 = t17 * zeta_threshold;
        let t51 = piecewise3::<f64>(2.0 <= zeta_threshold, t49, 2.0 * t15);
        let t53 = piecewise3::<f64>(0.0 <= zeta_threshold, t49, 0.0);
        let t57 = 1.0 / (2.0 * t15 - 2.0);
        let t58 = (t51 + t53 - 2.0) * t57;
        let t60 = 1.0 + 0.5137e-1 * t21;
        let t65 = 0.705945e1 * t24 + 0.1549425e1 * t21 + 0.420775e0 * t27 + 0.1562925e0 * t39;
        let t68 = 1.0 + 0.32164683177870697974e2 / t65;
        let t69 = f64::ln(t68);
        let t73 = 1.0 + 0.278125e-1 * t21;
        let t78 = 0.51785e1 * t24 + 0.905775e0 * t21 + 0.1100325e0 * t27 + 0.1241775e0 * t39;
        let t81 = 1.0 + 0.29608574643216675549e2 / t78;
        let t82 = f64::ln(t81);
        let t83 = t73 * t82;
        let t92 = piecewise3::<f64>(t4, 0.0, t5 * (-t47 + t58 * (-0.3109e-1 * t60 * t69 + t47 - 0.19751789702565206229e-1 * t83) + 0.19751789702565206229e-1 * t58 * t83) / 2.0);
        let t94 = param_c_ss_1;
        let t95 = t94 * sigma[ip];
        let t96 = rho[ip] * rho[ip];
        let t98 = 1.0 / t33 / t96;
        let t99 = t35 * t98;
        let t101 = sigma[ip] * t35 * t98;
        let t103 = 1.0 + 0.2e0 * t101;
        let t104 = 1.0 / t103;
        let t108 = param_c_ss_2;
        let t109 = sigma[ip] * sigma[ip];
        let t110 = t108 * t109;
        let t111 = t96 * t96;
        let t112 = t111 * rho[ip];
        let t114 = 1.0 / t13 / t112;
        let t115 = t15 * t114;
        let t116 = t103 * t103;
        let t117 = 1.0 / t116;
        let t118 = t115 * t117;
        let t121 = param_c_ss_3;
        let t122 = t109 * sigma[ip];
        let t123 = t121 * t122;
        let t124 = t111 * t111;
        let t125 = 1.0 / t124;
        let t126 = t116 * t103;
        let t127 = 1.0 / t126;
        let t128 = t125 * t127;
        let t131 = param_c_ss_4;
        let t132 = t109 * t109;
        let t133 = t131 * t132;
        let t134 = t124 * t96;
        let t136 = 1.0 / t33 / t134;
        let t137 = t35 * t136;
        let t138 = t116 * t116;
        let t139 = 1.0 / t138;
        let t140 = t137 * t139;
        let t143 = param_c_ss_0 + 0.2e0 * t95 * t99 * t104 + 0.8e-1 * t110 * t118 + 0.32e-1 * t123 * t128 + 0.64e-2 * t133 * t140;
        let t145 = 2.0 * t92 * t143;
        let t147 = t9 * t11 * t14;
        let t149 = 1.0 + 0.53425e-1 * t147;
        let t150 = f64::sqrt(t147);
        let t153 = pow_3_2::<f64>(t147);
        let t156 = t31 * t10 * t34;
        let t158 = 0.379785e1 * t150 + 0.8969e0 * t147 + 0.204775e0 * t153 + 0.123235e0 * t156;
        let t161 = 1.0 + 0.16081824322151104822e2 / t158;
        let t162 = f64::ln(t161);
        let t165 = piecewise3::<f64>(t3, t49, 1.0);
        let t168 = (2.0 * t165 - 2.0) * t57;
        let t170 = 1.0 + 0.278125e-1 * t147;
        let t175 = 0.51785e1 * t150 + 0.905775e0 * t147 + 0.1100325e0 * t153 + 0.1241775e0 * t156;
        let t178 = 1.0 + 0.29608574643216675549e2 / t175;
        let t179 = f64::ln(t178);
        let t184 = -0.62182e-1 * t149 * t162 + 0.19751789702565206229e-1 * t168 * t170 * t179 - 2.0 * t92;
        let t186 = param_c_ab_1;
        let t187 = t186 * sigma[ip];
        let t189 = 1.0 + 0.6e-2 * t101;
        let t190 = 1.0 / t189;
        let t194 = param_c_ab_2;
        let t195 = t194 * t109;
        let t196 = t189 * t189;
        let t197 = 1.0 / t196;
        let t198 = t115 * t197;
        let t201 = param_c_ab_3;
        let t202 = t201 * t122;
        let t203 = t196 * t189;
        let t204 = 1.0 / t203;
        let t205 = t125 * t204;
        let t208 = param_c_ab_4;
        let t209 = t208 * t132;
        let t210 = t196 * t196;
        let t211 = 1.0 / t210;
        let t212 = t137 * t211;
        let t215 = param_c_ab_0 + 0.6e-2 * t187 * t99 * t190 + 0.72e-4 * t195 * t198 + 0.864e-6 * t202 * t205 + 0.5184e-8 * t209 * t212;
        let t216 = t184 * t215;
        let tzk0 = t145 + t216;
        zk[ip] += tzk0;
    }
}
