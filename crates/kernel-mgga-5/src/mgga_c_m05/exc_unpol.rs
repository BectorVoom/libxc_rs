//! MGGA_C_M05 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 145 shared lines across all orders.
//! Delta: 145 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_m05_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_Fermi_D_cnst: f64,
    param_cab_0: f64,
    param_cab_1: f64,
    param_cab_2: f64,
    param_cab_3: f64,
    param_cab_4: f64,
    param_css_0: f64,
    param_css_1: f64,
    param_css_2: f64,
    param_css_3: f64,
    param_css_4: f64,
    param_gamma_ab: f64,
    param_gamma_ss: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (145 lines) ---
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
        let t95 = param_css_1;
        let t96 = t95 * param_gamma_ss;
        let t97 = t96 * sigma[ip];
        let t98 = rho[ip] * rho[ip];
        let t100 = 1.0 / t34 / t98;
        let t101 = t36 * t100;
        let t104 = param_gamma_ss * sigma[ip] * t101 + 1.0;
        let t105 = 1.0 / t104;
        let t106 = t101 * t105;
        let t108 = param_css_2;
        let t109 = param_gamma_ss * param_gamma_ss;
        let t110 = t108 * t109;
        let t111 = sigma[ip] * sigma[ip];
        let t112 = t110 * t111;
        let t113 = t98 * t98;
        let t114 = t113 * rho[ip];
        let t116 = 1.0 / t14 / t114;
        let t117 = t16 * t116;
        let t118 = t104 * t104;
        let t119 = 1.0 / t118;
        let t120 = t117 * t119;
        let t123 = param_css_3;
        let t124 = t109 * param_gamma_ss;
        let t125 = t123 * t124;
        let t126 = t111 * sigma[ip];
        let t127 = t113 * t113;
        let t128 = 1.0 / t127;
        let t129 = t126 * t128;
        let t130 = t118 * t104;
        let t131 = 1.0 / t130;
        let t135 = param_css_4;
        let t136 = t109 * t109;
        let t137 = t135 * t136;
        let t138 = t111 * t111;
        let t139 = t137 * t138;
        let t140 = t127 * t98;
        let t142 = 1.0 / t34 / t140;
        let t143 = t36 * t142;
        let t144 = t118 * t118;
        let t145 = 1.0 / t144;
        let t146 = t143 * t145;
        let t149 = 4.0 * t125 * t129 * t131 + t97 * t106 + 2.0 * t112 * t120 + 4.0 * t139 * t146 + param_css_0;
        let t150 = t93 * t149;
        let t151 = 1.0 / rho[ip];
        let t153 = 1.0 / tau[ip];
        let t156 = 1.0 - sigma[ip] * t151 * t153 / 8.0;
        let t157 = tau[ip] * tau[ip];
        let t159 = t98 * rho[ip];
        let t161 = 1.0 / t14 / t159;
        let t162 = param_Fermi_D_cnst * param_Fermi_D_cnst;
        let t163 = 1.0 / t162;
        let t167 = f64::exp(-8.0 * t157 * t16 * t161 * t163);
        let t168 = 1.0 - t167;
        let t169 = t156 * t168;
        let t171 = 2.0 * t150 * t169;
        let t173 = t10 * t12 * t15;
        let t175 = 1.0 + 0.53425e-1 * t173;
        let t176 = f64::sqrt(t173);
        let t179 = pow_3_2(t173);
        let t182 = t32 * t11 * t35;
        let t184 = 0.379785e1 * t176 + 0.8969e0 * t173 + 0.204775e0 * t179 + 0.123235e0 * t182;
        let t187 = 1.0 + 0.16081979498692535067e2 / t184;
        let t188 = f64::ln(t187);
        let t191 = piecewise3(t4, t50, 1.0);
        let t194 = (2.0 * t191 - 2.0) * t58;
        let t196 = 1.0 + 0.278125e-1 * t173;
        let t201 = 0.51785e1 * t176 + 0.905775e0 * t173 + 0.1100325e0 * t179 + 0.1241775e0 * t182;
        let t204 = 1.0 + 0.29608749977793437516e2 / t201;
        let t205 = f64::ln(t204);
        let t210 = -0.621814e-1 * t175 * t188 + 0.19751673498613801407e-1 * t194 * t196 * t205 - 2.0 * t93;
        let t212 = param_cab_1;
        let t213 = t212 * param_gamma_ab;
        let t214 = t213 * sigma[ip];
        let t218 = 2.0 * param_gamma_ab * sigma[ip] * t101 + 1.0;
        let t219 = 1.0 / t218;
        let t220 = t101 * t219;
        let t223 = param_cab_2;
        let t224 = param_gamma_ab * param_gamma_ab;
        let t225 = t223 * t224;
        let t226 = t225 * t111;
        let t227 = t218 * t218;
        let t228 = 1.0 / t227;
        let t229 = t117 * t228;
        let t232 = param_cab_3;
        let t233 = t224 * param_gamma_ab;
        let t234 = t232 * t233;
        let t235 = t227 * t218;
        let t236 = 1.0 / t235;
        let t240 = param_cab_4;
        let t241 = t224 * t224;
        let t242 = t240 * t241;
        let t243 = t242 * t138;
        let t244 = t227 * t227;
        let t245 = 1.0 / t244;
        let t246 = t143 * t245;
        let t249 = 32.0 * t234 * t129 * t236 + 2.0 * t214 * t220 + 8.0 * t226 * t229 + 64.0 * t243 * t246 + param_cab_0;
        let t250 = t210 * t249;
        let tzk0 = t171 + t250;
        zk[ip] += tzk0;
    }
}
