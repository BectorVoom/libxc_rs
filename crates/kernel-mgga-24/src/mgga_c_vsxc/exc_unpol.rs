//! MGGA_C_VSXC exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_vsxc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_c_vsxc_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha_ab: f64,
    param_alpha_ss: f64,
    param_dab_0: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    param_dss_0: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t4 = 1.0 <= zeta_threshold;
        let t5 = rho[ip] / 2.0 <= dens_threshold || t4;
        let t6 = piecewise3(t4, zeta_threshold, 1.0);
        let t7 = M_CBRT3;
        let t8 = 1.0 / M_PI;
        let t9 = pow_1_3(t8);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = t10 * t12;
        let t14 = pow_1_3(rho[ip]);
        let t15 = 1.0 / t14;
        let t16 = M_CBRT2;
        let t18 = pow_1_3(zeta_threshold);
        let t20 = piecewise3(t4, 1.0 / t18, 1.0);
        let t22 = t13 * t15 * t16 * t20;
        let t24 = 1.0 + 0.53425e-1 * t22;
        let t25 = f64::sqrt(t22);
        let t28 = pow_3_2(t22);
        let t30 = t7 * t7;
        let t31 = t9 * t9;
        let t32 = t30 * t31;
        let t33 = t32 * t11;
        let t34 = t14 * t14;
        let t35 = 1.0 / t34;
        let t36 = t16 * t16;
        let t38 = t20 * t20;
        let t40 = t33 * t35 * t36 * t38;
        let t42 = 0.379785e1 * t25 + 0.8969e0 * t22 + 0.204775e0 * t28 + 0.123235e0 * t40;
        let t45 = 1.0 + 0.16081979498692535067e2 / t42;
        let t46 = f64::ln(t45);
        let t48 = 0.621814e-1 * t24 * t46;
        let t50 = t18 * zeta_threshold;
        let t52 = piecewise3(2.0 <= zeta_threshold, t50, 2.0 * t16);
        let t54 = piecewise3(0.0 <= zeta_threshold, t50, 0.0);
        let t58 = 1.0 / (2.0 * t16 - 2.0);
        let t59 = (t52 + t54 - 2.0) * t58;
        let t61 = 1.0 + 0.5137e-1 * t22;
        let t66 = 0.705945e1 * t25 + 0.1549425e1 * t22 + 0.420775e0 * t28 + 0.1562925e0 * t40;
        let t69 = 1.0 + 0.32163958997385070134e2 / t66;
        let t70 = f64::ln(t69);
        let t74 = 1.0 + 0.278125e-1 * t22;
        let t79 = 0.51785e1 * t25 + 0.905775e0 * t22 + 0.1100325e0 * t28 + 0.1241775e0 * t40;
        let t82 = 1.0 + 0.29608749977793437516e2 / t79;
        let t83 = f64::ln(t82);
        let t84 = t74 * t83;
        let t93 = piecewise3(t5, 0.0, t6 * (-t48 + t59 * (-0.310907e-1 * t61 * t70 + t48 - 0.19751673498613801407e-1 * t84) + 0.19751673498613801407e-1 * t59 * t84) / 2.0);
        let t94 = param_dss_0;
        let t95 = sigma[ip] * t36;
        let t96 = rho[ip] * rho[ip];
        let t98 = 1.0 / t34 / t96;
        let t99 = t95 * t98;
        let t100 = tau[ip] * t36;
        let t102 = 1.0 / t34 / rho[ip];
        let t103 = t100 * t102;
        let t104 = 2.0 * t103;
        let t105 = M_CBRT6;
        let t106 = t105 * t105;
        let t107 = M_PI * M_PI;
        let t108 = pow_1_3(t107);
        let t109 = t108 * t108;
        let t110 = t106 * t109;
        let t111 = 3.0 / 5.0 * t110;
        let t114 = 1.0 + param_alpha_ss * (t99 + t104 - t111);
        let t117 = param_dss_1;
        let t118 = t117 * sigma[ip];
        let t119 = t36 * t98;
        let t121 = param_dss_2;
        let t122 = t104 - t111;
        let t124 = t118 * t119 + t121 * t122;
        let t125 = t114 * t114;
        let t126 = 1.0 / t125;
        let t128 = param_dss_3;
        let t129 = sigma[ip] * sigma[ip];
        let t130 = t128 * t129;
        let t131 = t96 * t96;
        let t132 = t131 * rho[ip];
        let t134 = 1.0 / t14 / t132;
        let t135 = t16 * t134;
        let t138 = param_dss_4;
        let t139 = t138 * sigma[ip];
        let t142 = param_dss_5;
        let t143 = t122 * t122;
        let t145 = t119 * t122 * t139 + 2.0 * t130 * t135 + t142 * t143;
        let t146 = t125 * t114;
        let t147 = 1.0 / t146;
        let t149 = t94 / t114 + t124 * t126 + t145 * t147;
        let t150 = t93 * t149;
        let t151 = 1.0 / rho[ip];
        let t152 = sigma[ip] * t151;
        let t153 = 1.0 / tau[ip];
        let t156 = 1.0 - t152 * t153 / 8.0;
        let t158 = 2.0 * t150 * t156;
        let t160 = t10 * t12 * t15;
        let t162 = 1.0 + 0.53425e-1 * t160;
        let t163 = f64::sqrt(t160);
        let t166 = pow_3_2(t160);
        let t169 = t32 * t11 * t35;
        let t171 = 0.379785e1 * t163 + 0.8969e0 * t160 + 0.204775e0 * t166 + 0.123235e0 * t169;
        let t174 = 1.0 + 0.16081979498692535067e2 / t171;
        let t175 = f64::ln(t174);
        let t178 = piecewise3(t4, t50, 1.0);
        let t181 = (2.0 * t178 - 2.0) * t58;
        let t183 = 1.0 + 0.278125e-1 * t160;
        let t188 = 0.51785e1 * t163 + 0.905775e0 * t160 + 0.1100325e0 * t166 + 0.1241775e0 * t169;
        let t191 = 1.0 + 0.29608749977793437516e2 / t188;
        let t192 = f64::ln(t191);
        let t197 = -0.621814e-1 * t162 * t175 + 0.19751673498613801407e-1 * t181 * t183 * t192 - 2.0 * t93;
        let t198 = param_dab_0;
        let t200 = 4.0 * t103;
        let t201 = 6.0 / 5.0 * t110;
        let t204 = 1.0 + param_alpha_ab * (2.0 * t99 + t200 - t201);
        let t207 = param_dab_1;
        let t208 = t207 * sigma[ip];
        let t211 = param_dab_2;
        let t212 = t200 - t201;
        let t214 = 2.0 * t119 * t208 + t211 * t212;
        let t215 = t204 * t204;
        let t216 = 1.0 / t215;
        let t218 = param_dab_3;
        let t219 = t218 * t129;
        let t222 = param_dab_4;
        let t223 = t222 * sigma[ip];
        let t227 = param_dab_5;
        let t228 = t212 * t212;
        let t230 = 2.0 * t119 * t212 * t223 + 8.0 * t135 * t219 + t227 * t228;
        let t231 = t215 * t204;
        let t232 = 1.0 / t231;
        let t234 = t198 / t204 + t214 * t216 + t230 * t232;
        let t235 = t197 * t234;
        let tzk0 = t158 + t235;
        zk[ip] += tzk0;
    }
}
