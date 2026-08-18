//! MGGA_X_SCAN exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_scan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_scan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t38 = t37 * t35;
        let t39 = 1.0 / t38;
        let t40 = sigma0 * t39;
        let t41 = t34 * t40;
        let t45 = 100.0 / 6561.0 / param_k1 - 73.0 / 648.0;
        let t46 = t29 * t29;
        let t47 = t45 * t46;
        let t48 = t31 * t30;
        let t49 = 1.0 / t48;
        let t50 = t47 * t49;
        let t51 = sigma0 * sigma0;
        let t52 = t35 * t35;
        let t53 = t52 * rho0;
        let t55 = 1.0 / t36 / t53;
        let t56 = t51 * t55;
        let t57 = t45 * t29;
        let t58 = t33 * sigma0;
        let t59 = t58 * t39;
        let t62 = f64::exp(-27.0 / 80.0 * t57 * t59);
        let t66 = f64::sqrt(146.0);
        let t67 = t66 * t29;
        let t70 = t37 * rho0;
        let t71 = 1.0 / t70;
        let t77 = 5.0 / 9.0 * (tau0 * t71 - t40 / 8.0) * t29 * t33;
        let t78 = 1.0 - t77;
        let t80 = t78 * t78;
        let t82 = f64::exp(-t80 / 2.0);
        let t85 = 7.0 / 12960.0 * t67 * t59 + t66 * t78 * t82 / 100.0;
        let t86 = t85 * t85;
        let t87 = param_k1 + 5.0 / 972.0 * t41 + t50 * t56 * t62 / 576.0 + t86;
        let t92 = 1.0 + param_k1 * (1.0 - param_k1 / t87);
        let t93 = t77 <= 1.0;
        let t94 = f64::ln(f64::EPSILON);
        let t97 = t94 / (-t94 + param_c1);
        let t98 = -t97 < t77;
        let t99 = t77 < -t97;
        let t100 = piecewise3(t99, t77, -t97);
        let t101 = param_c1 * t100;
        let t102 = 1.0 - t100;
        let t103 = 1.0 / t102;
        let t105 = f64::exp(-t101 * t103);
        let t106 = piecewise3(t98, 0.0, t105);
        let t107 = f64::abs(param_d);
        let t110 = f64::ln(f64::EPSILON / t107);
        let t113 = (-t110 + param_c2) / t110;
        let t114 = t77 < -t113;
        let t115 = piecewise3(t114, -t113, t77);
        let t116 = 1.0 - t115;
        let t119 = f64::exp(param_c2 / t116);
        let t121 = piecewise3(t114, 0.0, -param_d * t119);
        let t122 = piecewise3(t93, t106, t121);
        let t123 = 1.0 - t122;
        let t126 = t92 * t123 + 1.174 * t122;
        let t127 = t28 * t126;
        let t128 = f64::sqrt(3.0);
        let t129 = 1.0 / t31;
        let t130 = t46 * t129;
        let t131 = f64::sqrt(sigma0);
        let t132 = t36 * rho0;
        let t133 = 1.0 / t132;
        let t135 = t130 * t131 * t133;
        let t136 = f64::sqrt(t135);
        let t140 = f64::exp(-9.8958 * t128 / t136);
        let t141 = 1.0 - t140;
        let t142 = t127 * t141;
        let t145 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t142);
        let t146 = rho1 <= dens_threshold;
        let t147 = -t17;
        let t149 = piecewise5(t15, t12, t11, t16, t147 * t8);
        let t150 = 1.0 + t149;
        let t151 = t150 <= zeta_threshold;
        let t152 = pow_1_3(t150);
        let t154 = piecewise3(t151, t23, t152 * t150);
        let t155 = t6 * t154;
        let t156 = rho1 * rho1;
        let t157 = pow_1_3(rho1);
        let t158 = t157 * t157;
        let t159 = t158 * t156;
        let t160 = 1.0 / t159;
        let t161 = sigma2 * t160;
        let t162 = t34 * t161;
        let t164 = sigma2 * sigma2;
        let t165 = t156 * t156;
        let t166 = t165 * rho1;
        let t168 = 1.0 / t157 / t166;
        let t169 = t164 * t168;
        let t170 = t33 * sigma2;
        let t171 = t170 * t160;
        let t174 = f64::exp(-27.0 / 80.0 * t57 * t171);
        let t180 = t158 * rho1;
        let t181 = 1.0 / t180;
        let t187 = 5.0 / 9.0 * (tau1 * t181 - t161 / 8.0) * t29 * t33;
        let t188 = 1.0 - t187;
        let t190 = t188 * t188;
        let t192 = f64::exp(-t190 / 2.0);
        let t195 = 7.0 / 12960.0 * t67 * t171 + t66 * t188 * t192 / 100.0;
        let t196 = t195 * t195;
        let t197 = param_k1 + 5.0 / 972.0 * t162 + t50 * t169 * t174 / 576.0 + t196;
        let t202 = 1.0 + param_k1 * (1.0 - param_k1 / t197);
        let t203 = t187 <= 1.0;
        let t204 = -t97 < t187;
        let t205 = t187 < -t97;
        let t206 = piecewise3(t205, t187, -t97);
        let t207 = param_c1 * t206;
        let t208 = 1.0 - t206;
        let t209 = 1.0 / t208;
        let t211 = f64::exp(-t207 * t209);
        let t212 = piecewise3(t204, 0.0, t211);
        let t213 = t187 < -t113;
        let t214 = piecewise3(t213, -t113, t187);
        let t215 = 1.0 - t214;
        let t218 = f64::exp(param_c2 / t215);
        let t220 = piecewise3(t213, 0.0, -param_d * t218);
        let t221 = piecewise3(t203, t212, t220);
        let t222 = 1.0 - t221;
        let t225 = t202 * t222 + 1.174 * t221;
        let t226 = t28 * t225;
        let t227 = f64::sqrt(sigma2);
        let t228 = t157 * rho1;
        let t229 = 1.0 / t228;
        let t231 = t130 * t227 * t229;
        let t232 = f64::sqrt(t231);
        let t236 = f64::exp(-9.8958 * t128 / t232);
        let t237 = 1.0 - t236;
        let t238 = t226 * t237;
        let t241 = piecewise3(t146, 0.0, -3.0 / 8.0 * t155 * t238);
        let tzk0 = t145 + t241;
        zk[ip] += tzk0;
    }
}
