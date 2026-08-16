//! MGGA_X_RTPSS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rtpss.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rtpss_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
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
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = sigma0 * sigma0;
        let t30 = param_c * t29;
        let t31 = rho0 * rho0;
        let t32 = 1.0 / t31;
        let t33 = tau0 * tau0;
        let t34 = 1.0 / t33;
        let t35 = t32 * t34;
        let t36 = t29 * t32;
        let t37 = t36 * t34;
        let t39 = 1.0 + t37 / 64.0;
        let t40 = t39 * t39;
        let t41 = 1.0 / t40;
        let t42 = t35 * t41;
        let t46 = M_CBRT6;
        let t47 = (10.0 / 81.0 + t30 * t42 / 64.0) * t46;
        let t48 = M_PI * M_PI;
        let t49 = pow_1_3(t48);
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t51 * sigma0;
        let t53 = pow_1_3(rho0);
        let t54 = t53 * t53;
        let t56 = 1.0 / t54 / t31;
        let t57 = t52 * t56;
        let t61 = 1.0 / t54 / rho0;
        let t63 = sigma0 * t56;
        let t65 = tau0 * t61 - t63 / 8.0;
        let t69 = 5.0 / 9.0 * t65 * t46 * t51 - 1.0;
        let t70 = param_b * t65;
        let t71 = t46 * t51;
        let t72 = t71 * t69;
        let t75 = 5.0 * t70 * t72 + 9.0;
        let t76 = f64::sqrt(t75);
        let t77 = 1.0 / t76;
        let t82 = 27.0 / 20.0 * t69 * t77 + t71 * t63 / 36.0;
        let t83 = t82 * t82;
        let t86 = t46 * t46;
        let t88 = 1.0 / t49 / t48;
        let t89 = t86 * t88;
        let t90 = t31 * t31;
        let t91 = t90 * rho0;
        let t93 = 1.0 / t53 / t91;
        let t97 = 50.0 * t89 * t29 * t93 + 162.0 * t37;
        let t98 = f64::sqrt(t97);
        let t101 = 1.0 / param_kappa;
        let t102 = t101 * t86;
        let t103 = t88 * t29;
        let t107 = f64::sqrt(param_e);
        let t108 = t107 * t29;
        let t111 = param_e * param_mu;
        let t112 = t48 * t48;
        let t113 = 1.0 / t112;
        let t114 = t29 * sigma0;
        let t115 = t113 * t114;
        let t116 = t90 * t90;
        let t117 = 1.0 / t116;
        let t121 = t47 * t57 / 24.0 + 146.0 / 2025.0 * t83 - 73.0 / 97200.0 * t82 * t98 + 25.0 / 944784.0 * t102 * t103 * t93 + t108 * t35 / 720.0 + t111 * t115 * t117 / 2304.0;
        let t122 = t107 * t46;
        let t125 = 1.0 + t122 * t57 / 24.0;
        let t126 = t125 * t125;
        let t127 = 1.0 / t126;
        let t130 = f64::exp(-t121 * t127 * t101);
        let t133 = 1.0 + param_kappa * (1.0 - t130);
        let t137 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t133);
        let t138 = rho1 <= dens_threshold;
        let t139 = -t17;
        let t141 = piecewise5(t15, t12, t11, t16, t139 * t8);
        let t142 = 1.0 + t141;
        let t143 = t142 <= zeta_threshold;
        let t144 = pow_1_3(t142);
        let t146 = piecewise3(t143, t23, t144 * t142);
        let t147 = t146 * t27;
        let t148 = sigma2 * sigma2;
        let t149 = param_c * t148;
        let t150 = rho1 * rho1;
        let t151 = 1.0 / t150;
        let t152 = tau1 * tau1;
        let t153 = 1.0 / t152;
        let t154 = t151 * t153;
        let t155 = t148 * t151;
        let t156 = t155 * t153;
        let t158 = 1.0 + t156 / 64.0;
        let t159 = t158 * t158;
        let t160 = 1.0 / t159;
        let t161 = t154 * t160;
        let t165 = (10.0 / 81.0 + t149 * t161 / 64.0) * t46;
        let t166 = t51 * sigma2;
        let t167 = pow_1_3(rho1);
        let t168 = t167 * t167;
        let t170 = 1.0 / t168 / t150;
        let t171 = t166 * t170;
        let t175 = 1.0 / t168 / rho1;
        let t177 = sigma2 * t170;
        let t179 = tau1 * t175 - t177 / 8.0;
        let t183 = 5.0 / 9.0 * t179 * t46 * t51 - 1.0;
        let t184 = param_b * t179;
        let t185 = t71 * t183;
        let t188 = 5.0 * t184 * t185 + 9.0;
        let t189 = f64::sqrt(t188);
        let t190 = 1.0 / t189;
        let t195 = 27.0 / 20.0 * t183 * t190 + t71 * t177 / 36.0;
        let t196 = t195 * t195;
        let t199 = t150 * t150;
        let t200 = t199 * rho1;
        let t202 = 1.0 / t167 / t200;
        let t206 = 50.0 * t89 * t148 * t202 + 162.0 * t156;
        let t207 = f64::sqrt(t206);
        let t210 = t88 * t148;
        let t214 = t107 * t148;
        let t217 = t148 * sigma2;
        let t218 = t113 * t217;
        let t219 = t199 * t199;
        let t220 = 1.0 / t219;
        let t224 = t165 * t171 / 24.0 + 146.0 / 2025.0 * t196 - 73.0 / 97200.0 * t195 * t207 + 25.0 / 944784.0 * t102 * t210 * t202 + t214 * t154 / 720.0 + t111 * t218 * t220 / 2304.0;
        let t227 = 1.0 + t122 * t171 / 24.0;
        let t228 = t227 * t227;
        let t229 = 1.0 / t228;
        let t232 = f64::exp(-t224 * t229 * t101);
        let t235 = 1.0 + param_kappa * (1.0 - t232);
        let t239 = piecewise3(t138, 0.0, -3.0 / 8.0 * t6 * t147 * t235);
        let tzk0 = t137 + t239;
        zk[ip] += tzk0;
    }
}
