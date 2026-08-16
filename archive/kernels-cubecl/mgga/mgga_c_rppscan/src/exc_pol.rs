//! MGGA_C_RPPSCAN exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rppscan.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_rppscan_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_eta: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3::<f64>(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = rho0 + rho1;
        let t9 = pow_1_3::<f64>(t8);
        let t12 = t5 * t7 / t9;
        let t14 = 1.0 + 0.53425e-1 * t12;
        let t15 = f64::sqrt(t12);
        let t18 = pow_3_2::<f64>(t12);
        let t20 = t2 * t2;
        let t21 = t4 * t4;
        let t22 = t20 * t21;
        let t23 = t9 * t9;
        let t26 = t22 * t6 / t23;
        let t28 = 0.379785e1 * t15 + 0.8969e0 * t12 + 0.204775e0 * t18 + 0.123235e0 * t26;
        let t31 = 1.0 + 0.16081979498692535067e2 / t28;
        let t32 = f64::ln(t31);
        let t34 = 0.621814e-1 * t14 * t32;
        let t35 = rho0 - rho1;
        let t36 = t35 * t35;
        let t37 = t36 * t36;
        let t38 = t8 * t8;
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t37 * t40;
        let t42 = 1.0 / t8;
        let t43 = t35 * t42;
        let t44 = 1.0 + t43;
        let t45 = t44 <= zeta_threshold;
        let t46 = pow_1_3::<f64>(zeta_threshold);
        let t47 = t46 * zeta_threshold;
        let t48 = pow_1_3::<f64>(t44);
        let t49 = t48 * t44;
        let t50 = piecewise3::<f64>(t45, t47, t49);
        let t51 = 1.0 - t43;
        let t52 = t51 <= zeta_threshold;
        let t53 = pow_1_3::<f64>(t51);
        let t54 = t53 * t51;
        let t55 = piecewise3::<f64>(t52, t47, t54);
        let t56 = t50 + t55 - 2.0;
        let t57 = M_CBRT2;
        let t58 = t57 - 1.0;
        let t60 = 1.0 / t58 / 2.0;
        let t61 = t56 * t60;
        let t63 = 1.0 + 0.5137e-1 * t12;
        let t68 = 0.705945e1 * t15 + 0.1549425e1 * t12 + 0.420775e0 * t18 + 0.1562925e0 * t26;
        let t71 = 1.0 + 0.32163958997385070134e2 / t68;
        let t72 = f64::ln(t71);
        let t76 = 1.0 + 0.278125e-1 * t12;
        let t81 = 0.51785e1 * t15 + 0.905775e0 * t12 + 0.1100325e0 * t18 + 0.1241775e0 * t26;
        let t84 = 1.0 + 0.29608749977793437516e2 / t81;
        let t85 = f64::ln(t84);
        let t86 = t76 * t85;
        let t88 = -0.310907e-1 * t63 * t72 + t34 - 0.19751673498613801407e-1 * t86;
        let t89 = t61 * t88;
        let t90 = t41 * t89;
        let t92 = 0.19751673498613801407e-1 * t61 * t86;
        let t93 = f64::ln(2.0);
        let t94 = 1.0 - t93;
        let t95 = M_PI * M_PI;
        let t97 = t94 / t95;
        let t98 = t46 * t46;
        let t99 = t48 * t48;
        let t100 = piecewise3::<f64>(t45, t98, t99);
        let t101 = t53 * t53;
        let t102 = piecewise3::<f64>(t52, t98, t101);
        let t104 = t100 / 2.0 + t102 / 2.0;
        let t105 = t104 * t104;
        let t106 = t105 * t104;
        let t108 = 1.0 + 0.25e-1 * t12;
        let t110 = 1.0 + 0.4445e-1 * t12;
        let t111 = 1.0 / t110;
        let t112 = t108 * t111;
        let t113 = 1.0 / t94;
        let t115 = (-t34 + t90 + t92) * t113;
        let t116 = 1.0 / t106;
        let t117 = t95 * t116;
        let t119 = f64::exp(-t115 * t117);
        let t120 = t119 - 1.0;
        let t121 = 1.0 / t120;
        let t122 = t113 * t121;
        let t124 = sigma0 + 2.0 * sigma1 + sigma2;
        let t125 = t122 * t124;
        let t126 = t112 * t125;
        let t128 = 1.0 / t9 / t38;
        let t129 = t128 * t57;
        let t130 = 1.0 / t105;
        let t132 = 1.0 / t4;
        let t133 = t20 * t132;
        let t134 = t133 * t6;
        let t138 = 1.0 + 0.27439371595564631661e-1 * t126 * t129 * t130 * t134;
        let t139 = pow_1_4::<f64>(t138);
        let t141 = 1.0 - 1.0 / t139;
        let t144 = 1.0 + 1.0 * t141 * t120;
        let t145 = f64::ln(t144);
        let t147 = t97 * t106 * t145;
        let t148 = pow_1_3::<f64>(rho0);
        let t149 = t148 * t148;
        let t151 = 1.0 / t149 / rho0;
        let t152 = tau0 * t151;
        let t153 = t44 / 2.0;
        let t154 = pow_1_3::<f64>(t153);
        let t155 = t154 * t154;
        let t156 = t155 * t153;
        let t158 = pow_1_3::<f64>(rho1);
        let t159 = t158 * t158;
        let t161 = 1.0 / t159 / rho1;
        let t162 = tau1 * t161;
        let t163 = t51 / 2.0;
        let t164 = pow_1_3::<f64>(t163);
        let t165 = t164 * t164;
        let t166 = t165 * t163;
        let t169 = 1.0 / t23 / t38;
        let t172 = t152 * t156 + t162 * t166 - t124 * t169 / 8.0;
        let t173 = M_CBRT6;
        let t174 = t173 * t173;
        let t175 = pow_1_3::<f64>(t95);
        let t176 = t175 * t175;
        let t177 = t174 * t176;
        let t181 = param_eta * t124;
        let t184 = 3.0 / 10.0 * t177 * (t156 + t166) + t181 * t169 / 8.0;
        let t185 = 1.0 / t184;
        let t186 = t172 * t185;
        let t187 = t186 <= 0.25e1;
        let t188 = 0.25e1 < t186;
        let t189 = piecewise3::<f64>(t188, 0.25e1, t186);
        let t191 = t189 * t189;
        let t193 = t191 * t189;
        let t195 = t191 * t191;
        let t197 = t195 * t189;
        let t199 = t195 * t191;
        let t204 = piecewise3::<f64>(t188, t186, 0.25e1);
        let t205 = 1.0 - t204;
        let t208 = f64::exp(0.15e1 / t205);
        let t210 = piecewise3::<f64>(t187, 1.0 - 0.64e0 * t189 - 0.4352e0 * t191 - 0.1535685604549e1 * t193 + 0.3061560252175e1 * t195 - 0.1915710236206e1 * t197 + 0.516884468372e0 * t199 - 0.51848879792e-1 * t195 * t193, -0.7e0 * t208);
        let t213 = 1.0 + 0.4445e-1 * t15 + 0.3138525e-1 * t12;
        let t214 = 1.0 / t213;
        let t217 = f64::exp(1.0 * t214);
        let t218 = t217 - 1.0;
        let t219 = 1.0 / t176;
        let t220 = t173 * t219;
        let t221 = t57 * t57;
        let t222 = t221 * t124;
        let t226 = 1.0 + 0.21337642104376358333e-1 * t220 * t222 * t169;
        let t227 = pow_1_4::<f64>(t226);
        let t229 = 1.0 - 1.0 / t227;
        let t231 = t218 * t229 + 1.0;
        let t232 = f64::ln(t231);
        let t234 = -0.285764e-1 * t214 + 0.285764e-1 * t232;
        let t238 = 1.0 - 0.2363e1 * t58 * t56 * t60;
        let t239 = t234 * t238;
        let t240 = t37 * t37;
        let t241 = t240 * t37;
        let t242 = t39 * t39;
        let t243 = t242 * t39;
        let t244 = 1.0 / t243;
        let t246 = -t241 * t244 + 1.0;
        let t248 = t239 * t246 - t147 + t34 - t90 - t92;
        let t249 = t210 * t248;
        let tzk0 = -t34 + t90 + t92 + t147 + t249;
        zk[ip] += tzk0;
    }
}
