//! MGGA_K_PC07 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pc07.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_pc07_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_b: f64,
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t45 = t38 * sigma0 * t43;
        let t46 = 5.0 / 72.0 * t45;
        let t49 = 1.0 / t41 / rho0;
        let t53 = t33 * t33;
        let t55 = 1.0 / t35 / t34;
        let t56 = t53 * t55;
        let t57 = lapl0 * lapl0;
        let t58 = t39 * rho0;
        let t60 = 1.0 / t40 / t58;
        let t63 = t56 * t57 * t60 / 5832.0;
        let t64 = t39 * t39;
        let t66 = 1.0 / t40 / t64;
        let t67 = sigma0 * t66;
        let t70 = t56 * t67 * lapl0 / 5184.0;
        let t71 = sigma0 * sigma0;
        let t72 = t64 * rho0;
        let t74 = 1.0 / t40 / t72;
        let t77 = t56 * t71 * t74 / 17496.0;
        let t78 = 1.0 + 5.0 / 648.0 * t45 + 5.0 / 54.0 * t38 * lapl0 * t49 + t63 - t70 + t77;
        let t79 = t63 - t70 + t77;
        let t80 = t79 * t79;
        let t81 = 1.0 + t46;
        let t82 = t81 * t81;
        let t83 = 1.0 / t82;
        let t85 = t80 * t83 + 1.0;
        let t86 = rmath::sqrt(t85);
        let t87 = 1.0 / t86;
        let t89 = t78 * t87 - t46;
        let t90 = param_a / 40.0;
        let t91 = t89 <= t90;
        let t92 = 39.0 / 40.0 * param_a;
        let t93 = t92 <= t89;
        let t94 = param_a * param_b;
        let t95 = t89 < t90;
        let t96 = piecewise3(t95, t90, t89);
        let t97 = t96 < t92;
        let t98 = piecewise3(t97, t96, t92);
        let t99 = 1.0 / t98;
        let t101 = rmath::exp(-t94 * t99);
        let t102 = param_a - t98;
        let t105 = rmath::exp(-param_a / t102);
        let t106 = 1.0 + t105;
        let t107 = rmath::pow(t106, param_b);
        let t108 = t101 * t107;
        let t110 = rmath::exp(-param_a * t99);
        let t111 = t110 + t105;
        let t112 = rmath::pow(t111, param_b);
        let t113 = 1.0 / t112;
        let t114 = t108 * t113;
        let t115 = piecewise5(t91, 0.0, t93, 1.0, t114);
        let t117 = t115 * t89 + t46;
        let t121 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t117);
        let t122 = rho1 <= dens_threshold;
        let t123 = -t18;
        let t125 = piecewise5(t16, t13, t12, t17, t123 * t9);
        let t126 = 1.0 + t125;
        let t127 = t126 <= zeta_threshold;
        let t128 = pow_1_3(t126);
        let t129 = t128 * t128;
        let t131 = piecewise3(t127, t25, t129 * t126);
        let t132 = t131 * t31;
        let t133 = rho1 * rho1;
        let t134 = pow_1_3(rho1);
        let t135 = t134 * t134;
        let t137 = 1.0 / t135 / t133;
        let t139 = t38 * sigma2 * t137;
        let t140 = 5.0 / 72.0 * t139;
        let t143 = 1.0 / t135 / rho1;
        let t147 = lapl1 * lapl1;
        let t148 = t133 * rho1;
        let t150 = 1.0 / t134 / t148;
        let t153 = t56 * t147 * t150 / 5832.0;
        let t154 = t133 * t133;
        let t156 = 1.0 / t134 / t154;
        let t157 = sigma2 * t156;
        let t160 = t56 * t157 * lapl1 / 5184.0;
        let t161 = sigma2 * sigma2;
        let t162 = t154 * rho1;
        let t164 = 1.0 / t134 / t162;
        let t167 = t56 * t161 * t164 / 17496.0;
        let t168 = 1.0 + 5.0 / 648.0 * t139 + 5.0 / 54.0 * t38 * lapl1 * t143 + t153 - t160 + t167;
        let t169 = t153 - t160 + t167;
        let t170 = t169 * t169;
        let t171 = 1.0 + t140;
        let t172 = t171 * t171;
        let t173 = 1.0 / t172;
        let t175 = t170 * t173 + 1.0;
        let t176 = rmath::sqrt(t175);
        let t177 = 1.0 / t176;
        let t179 = t168 * t177 - t140;
        let t180 = t179 <= t90;
        let t181 = t92 <= t179;
        let t182 = t179 < t90;
        let t183 = piecewise3(t182, t90, t179);
        let t184 = t183 < t92;
        let t185 = piecewise3(t184, t183, t92);
        let t186 = 1.0 / t185;
        let t188 = rmath::exp(-t94 * t186);
        let t189 = param_a - t185;
        let t192 = rmath::exp(-param_a / t189);
        let t193 = 1.0 + t192;
        let t194 = rmath::pow(t193, param_b);
        let t195 = t188 * t194;
        let t197 = rmath::exp(-param_a * t186);
        let t198 = t197 + t192;
        let t199 = rmath::pow(t198, param_b);
        let t200 = 1.0 / t199;
        let t201 = t195 * t200;
        let t202 = piecewise5(t180, 0.0, t181, 1.0, t201);
        let t204 = t179 * t202 + t140;
        let t208 = piecewise3(t122, 0.0, 3.0 / 20.0 * t7 * t132 * t204);
        let tzk0 = t121 + t208;
        zk[ip] += tzk0;
    }
}
