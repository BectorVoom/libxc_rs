//! MGGA_X_TPSS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tpss_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_BLOC_a: f64,
    param_BLOC_b: f64,
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
        let t29 = 1.0 / rho0;
        let t31 = 1.0 / tau0;
        let t33 = sigma0 * t29 * t31 / 8.0;
        let t34 = param_BLOC_b * sigma0;
        let t38 = param_BLOC_a + t34 * t29 * t31 / 8.0;
        let t39 = rmath::pow(t33, t38);
        let t40 = param_c * t39;
        let t41 = sigma0 * sigma0;
        let t42 = rho0 * rho0;
        let t43 = 1.0 / t42;
        let t44 = t41 * t43;
        let t45 = tau0 * tau0;
        let t46 = 1.0 / t45;
        let t47 = t44 * t46;
        let t49 = 1.0 + t47 / 64.0;
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t54 = M_CBRT6;
        let t55 = (10.0 / 81.0 + t40 * t51) * t54;
        let t56 = M_PI * M_PI;
        let t57 = pow_1_3(t56);
        let t58 = t57 * t57;
        let t59 = 1.0 / t58;
        let t60 = t59 * sigma0;
        let t61 = pow_1_3(rho0);
        let t62 = t61 * t61;
        let t64 = 1.0 / t62 / t42;
        let t65 = t60 * t64;
        let t69 = 1.0 / t62 / rho0;
        let t71 = sigma0 * t64;
        let t73 = tau0 * t69 - t71 / 8.0;
        let t77 = 5.0 / 9.0 * t73 * t54 * t59 - 1.0;
        let t78 = param_b * t73;
        let t79 = t54 * t59;
        let t80 = t79 * t77;
        let t83 = 5.0 * t78 * t80 + 9.0;
        let t84 = rmath::sqrt(t83);
        let t85 = 1.0 / t84;
        let t90 = 27.0 / 20.0 * t77 * t85 + t79 * t71 / 36.0;
        let t91 = t90 * t90;
        let t94 = t54 * t54;
        let t96 = 1.0 / t57 / t56;
        let t97 = t94 * t96;
        let t98 = t42 * t42;
        let t99 = t98 * rho0;
        let t101 = 1.0 / t61 / t99;
        let t105 = 50.0 * t97 * t41 * t101 + 162.0 * t47;
        let t106 = rmath::sqrt(t105);
        let t110 = 1.0 / param_kappa * t94;
        let t111 = t96 * t41;
        let t115 = rmath::sqrt(param_e);
        let t116 = t115 * t41;
        let t117 = t43 * t46;
        let t120 = param_e * param_mu;
        let t121 = t56 * t56;
        let t122 = 1.0 / t121;
        let t123 = t41 * sigma0;
        let t124 = t122 * t123;
        let t125 = t98 * t98;
        let t126 = 1.0 / t125;
        let t130 = t55 * t65 / 24.0 + 146.0 / 2025.0 * t91 - 73.0 / 97200.0 * t90 * t106 + 25.0 / 944784.0 * t110 * t111 * t101 + t116 * t117 / 720.0 + t120 * t124 * t126 / 2304.0;
        let t131 = t115 * t54;
        let t134 = 1.0 + t131 * t65 / 24.0;
        let t135 = t134 * t134;
        let t136 = 1.0 / t135;
        let t138 = t130 * t136 + param_kappa;
        let t143 = 1.0 + param_kappa * (1.0 - param_kappa / t138);
        let t147 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t143);
        let t148 = rho1 <= dens_threshold;
        let t149 = -t17;
        let t151 = piecewise5(t15, t12, t11, t16, t149 * t8);
        let t152 = 1.0 + t151;
        let t153 = t152 <= zeta_threshold;
        let t154 = pow_1_3(t152);
        let t156 = piecewise3(t153, t23, t154 * t152);
        let t157 = t156 * t27;
        let t158 = 1.0 / rho1;
        let t160 = 1.0 / tau1;
        let t162 = sigma2 * t158 * t160 / 8.0;
        let t163 = param_BLOC_b * sigma2;
        let t167 = param_BLOC_a + t163 * t158 * t160 / 8.0;
        let t168 = rmath::pow(t162, t167);
        let t169 = param_c * t168;
        let t170 = sigma2 * sigma2;
        let t171 = rho1 * rho1;
        let t172 = 1.0 / t171;
        let t173 = t170 * t172;
        let t174 = tau1 * tau1;
        let t175 = 1.0 / t174;
        let t176 = t173 * t175;
        let t178 = 1.0 + t176 / 64.0;
        let t179 = t178 * t178;
        let t180 = 1.0 / t179;
        let t183 = (10.0 / 81.0 + t169 * t180) * t54;
        let t184 = t59 * sigma2;
        let t185 = pow_1_3(rho1);
        let t186 = t185 * t185;
        let t188 = 1.0 / t186 / t171;
        let t189 = t184 * t188;
        let t193 = 1.0 / t186 / rho1;
        let t195 = sigma2 * t188;
        let t197 = tau1 * t193 - t195 / 8.0;
        let t201 = 5.0 / 9.0 * t197 * t54 * t59 - 1.0;
        let t202 = param_b * t197;
        let t203 = t79 * t201;
        let t206 = 5.0 * t202 * t203 + 9.0;
        let t207 = rmath::sqrt(t206);
        let t208 = 1.0 / t207;
        let t213 = 27.0 / 20.0 * t201 * t208 + t79 * t195 / 36.0;
        let t214 = t213 * t213;
        let t217 = t171 * t171;
        let t218 = t217 * rho1;
        let t220 = 1.0 / t185 / t218;
        let t224 = 50.0 * t97 * t170 * t220 + 162.0 * t176;
        let t225 = rmath::sqrt(t224);
        let t228 = t96 * t170;
        let t232 = t115 * t170;
        let t233 = t172 * t175;
        let t236 = t170 * sigma2;
        let t237 = t122 * t236;
        let t238 = t217 * t217;
        let t239 = 1.0 / t238;
        let t243 = t183 * t189 / 24.0 + 146.0 / 2025.0 * t214 - 73.0 / 97200.0 * t213 * t225 + 25.0 / 944784.0 * t110 * t228 * t220 + t232 * t233 / 720.0 + t120 * t237 * t239 / 2304.0;
        let t246 = 1.0 + t131 * t189 / 24.0;
        let t247 = t246 * t246;
        let t248 = 1.0 / t247;
        let t250 = t243 * t248 + param_kappa;
        let t255 = 1.0 + param_kappa * (1.0 - param_kappa / t250);
        let t259 = piecewise3(t148, 0.0, -3.0 / 8.0 * t6 * t157 * t255);
        let tzk0 = t147 + t259;
        zk[ip] += tzk0;
    }
}
