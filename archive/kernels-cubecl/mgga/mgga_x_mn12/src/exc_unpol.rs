//! MGGA_X_MN12 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mn12.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mn12_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c_0: f64,
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_6: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_11: f64,
    param_c_12: f64,
    param_c_13: f64,
    param_c_14: f64,
    param_c_15: f64,
    param_c_16: f64,
    param_c_17: f64,
    param_c_18: f64,
    param_c_19: f64,
    param_c_20: f64,
    param_c_21: f64,
    param_c_22: f64,
    param_c_23: f64,
    param_c_24: f64,
    param_c_25: f64,
    param_c_26: f64,
    param_c_27: f64,
    param_c_28: f64,
    param_c_29: f64,
    param_c_30: f64,
    param_c_31: f64,
    param_c_32: f64,
    param_c_33: f64,
    param_c_34: f64,
    param_c_35: f64,
    param_c_36: f64,
    param_c_37: f64,
    param_c_38: f64,
    param_c_39: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t22 = param_c_1;
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3::<f64>(t25);
        let t27 = t26 * t26;
        let t29 = 3.0 / 10.0 * t24 * t27;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = tau[ip] * t31;
        let t33 = t19 * t19;
        let t35 = 1.0 / t33 / rho[ip];
        let t36 = t32 * t35;
        let t37 = t29 - t36;
        let t38 = t22 * t37;
        let t39 = t29 + t36;
        let t40 = 1.0 / t39;
        let t42 = param_c_2;
        let t43 = t37 * t37;
        let t44 = t42 * t43;
        let t45 = t39 * t39;
        let t46 = 1.0 / t45;
        let t48 = param_c_3;
        let t49 = t43 * t37;
        let t50 = t48 * t49;
        let t51 = t45 * t39;
        let t52 = 1.0 / t51;
        let t54 = param_c_4;
        let t55 = t43 * t43;
        let t56 = t54 * t55;
        let t57 = t45 * t45;
        let t58 = 1.0 / t57;
        let t60 = param_c_5;
        let t62 = t60 * t55 * t37;
        let t64 = 1.0 / t57 / t39;
        let t67 = param_c_7;
        let t68 = t67 * t37;
        let t70 = param_c_8;
        let t71 = t70 * t43;
        let t73 = param_c_9;
        let t74 = t73 * t49;
        let t76 = param_c_10;
        let t77 = t76 * t55;
        let t79 = t68 * t40 + t71 * t46 + t74 * t52 + t77 * t58 + param_c_6;
        let t80 = t79 * sigma[ip];
        let t81 = rho[ip] * rho[ip];
        let t83 = 1.0 / t33 / t81;
        let t84 = t31 * t83;
        let t85 = sigma[ip] * t31;
        let t88 = 1.0 + 0.4e-2 * t85 * t83;
        let t89 = 1.0 / t88;
        let t90 = t84 * t89;
        let t94 = param_c_12;
        let t95 = t94 * t37;
        let t97 = param_c_13;
        let t98 = t97 * t43;
        let t100 = param_c_14;
        let t101 = t100 * t49;
        let t103 = t101 * t52 + t95 * t40 + t98 * t46 + param_c_11;
        let t104 = sigma[ip] * sigma[ip];
        let t105 = t103 * t104;
        let t106 = t81 * t81;
        let t107 = t106 * rho[ip];
        let t109 = 1.0 / t19 / t107;
        let t110 = t30 * t109;
        let t111 = t88 * t88;
        let t112 = 1.0 / t111;
        let t113 = t110 * t112;
        let t117 = param_c_16;
        let t118 = t117 * t37;
        let t120 = param_c_17;
        let t121 = t120 * t43;
        let t123 = t118 * t40 + t121 * t46 + param_c_15;
        let t124 = t104 * sigma[ip];
        let t125 = t123 * t124;
        let t126 = t106 * t106;
        let t127 = 1.0 / t126;
        let t128 = t111 * t88;
        let t129 = 1.0 / t128;
        let t130 = t127 * t129;
        let t134 = param_c_19;
        let t135 = t134 * t37;
        let t137 = param_c_20;
        let t138 = t137 * t43;
        let t140 = param_c_21;
        let t141 = t140 * t49;
        let t143 = param_c_22;
        let t144 = t143 * t55;
        let t146 = t135 * t40 + t138 * t46 + t141 * t52 + t144 * t58 + param_c_18;
        let t149 = 1.0 / t12;
        let t150 = pow_1_3::<f64>(t149);
        let t153 = 1.0 + 0.39999999999999999998e0 / t19 * t30 * t150;
        let t154 = 1.0 / t153;
        let t157 = param_c_24;
        let t158 = t157 * t37;
        let t160 = param_c_25;
        let t161 = t160 * t43;
        let t163 = param_c_26;
        let t164 = t163 * t49;
        let t166 = t158 * t40 + t161 * t46 + t164 * t52 + param_c_23;
        let t167 = t166 * sigma[ip];
        let t168 = t167 * t31;
        let t169 = t83 * t89;
        let t170 = t169 * t154;
        let t174 = param_c_28;
        let t175 = t174 * t37;
        let t177 = param_c_29;
        let t178 = t177 * t43;
        let t180 = t175 * t40 + t178 * t46 + param_c_27;
        let t181 = t180 * t104;
        let t182 = t181 * t30;
        let t183 = t109 * t112;
        let t184 = t183 * t154;
        let t188 = param_c_31;
        let t189 = t188 * t37;
        let t191 = param_c_32;
        let t192 = t191 * t43;
        let t194 = param_c_33;
        let t195 = t194 * t49;
        let t197 = t189 * t40 + t192 * t46 + t195 * t52 + param_c_30;
        let t198 = t153 * t153;
        let t199 = 1.0 / t198;
        let t202 = param_c_35;
        let t203 = t202 * t37;
        let t205 = param_c_36;
        let t206 = t205 * t43;
        let t208 = t203 * t40 + t206 * t46 + param_c_34;
        let t209 = t208 * sigma[ip];
        let t210 = t209 * t31;
        let t211 = t169 * t199;
        let t215 = param_c_38;
        let t216 = t215 * t37;
        let t218 = param_c_39;
        let t219 = t218 * t43;
        let t221 = t216 * t40 + t219 * t46 + param_c_37;
        let t222 = t198 * t153;
        let t223 = 1.0 / t222;
        let t225 = param_c_0 + t38 * t40 + t44 * t46 + t50 * t52 + t56 * t58 + t62 * t64 + 0.4e-2 * t80 * t90 + 0.32e-4 * t105 * t113 + 0.256e-6 * t125 * t130 + t146 * t154 + 0.4e-2 * t168 * t170 + 0.32e-4 * t182 * t184 + t197 * t199 + 0.4e-2 * t210 * t211 + t221 * t223;
        let t229 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t225);
        let tzk0 = 2.0 * t229;
        zk[ip] += tzk0;
    }
}
