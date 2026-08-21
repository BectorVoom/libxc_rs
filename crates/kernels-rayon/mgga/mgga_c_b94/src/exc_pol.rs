//! MGGA_C_B94 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRTPI};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_b94_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
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
        let t2 = rho0 - rho1;
        let t3 = t2 * t2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = 1.0 / t5;
        let t8 = -t3 * t6 + 1.0;
        let t9 = t8 * t4;
        let t10 = rho0 <= dens_threshold;
        let t11 = M_CBRT2;
        let t12 = 1.0 / t4;
        let t15 = 2.0 * rho0 * t12 <= zeta_threshold;
        let t16 = zeta_threshold - 1.0;
        let t19 = 2.0 * rho1 * t12 <= zeta_threshold;
        let t20 = -t16;
        let t21 = t2 * t12;
        let t22 = piecewise5(t15, t16, t19, t20, t21);
        let t23 = 1.0 + t22;
        let t24 = t23 * t4;
        let t25 = pow_1_3(t24);
        let t26 = 1.0 / t25;
        let t27 = t11 * t26;
        let t28 = M_CBRTPI;
        let t29 = 1.0 / t28;
        let t30 = t27 * t29;
        let t31 = pow_1_3(rho0);
        let t32 = t31 * t31;
        let t34 = 1.0 / t32 / rho0;
        let t35 = lapl0 * t34;
        let t37 = param_gamma * tau0;
        let t38 = t37 * t34;
        let t40 = param_gamma * sigma0;
        let t41 = rho0 * rho0;
        let t43 = 1.0 / t32 / t41;
        let t44 = t40 * t43;
        let t47 = rmath::abs(t35 / 2.0 - 2.0 * t38 + t44 / 4.0);
        let t49 = t47 / 3.0 < 5e-13;
        let t53 = t35 / 6.0 - 2.0 / 3.0 * t38 + t44 / 12.0;
        let t54 = 0.0 < t53;
        let t55 = piecewise3(t54, 5e-13, -5e-13);
        let t56 = piecewise3(t49, t55, t53);
        let t57 = xc_mgga_x_br89_get_x(t56);
        let t59 = rmath::exp(t57 / 3.0);
        let t60 = 1.0 / t59;
        let t61 = rmath::exp(-t57);
        let t63 = 1.0 + t57 / 2.0;
        let t64 = t61 * t63;
        let t65 = 1.0 - t64;
        let t66 = 1.0 / t65;
        let t67 = t60 * t66;
        let t68 = t67 * t57;
        let t71 = piecewise3(t10, 0.0, t30 * t68 / 2.0);
        let t72 = rho1 <= dens_threshold;
        let t73 = -t2;
        let t75 = piecewise5(t19, t16, t15, t20, t73 * t12);
        let t76 = 1.0 + t75;
        let t77 = t76 * t4;
        let t78 = pow_1_3(t77);
        let t79 = 1.0 / t78;
        let t80 = t11 * t79;
        let t81 = t80 * t29;
        let t82 = pow_1_3(rho1);
        let t83 = t82 * t82;
        let t85 = 1.0 / t83 / rho1;
        let t86 = lapl1 * t85;
        let t88 = param_gamma * tau1;
        let t89 = t88 * t85;
        let t91 = param_gamma * sigma2;
        let t92 = rho1 * rho1;
        let t94 = 1.0 / t83 / t92;
        let t95 = t91 * t94;
        let t98 = rmath::abs(t86 / 2.0 - 2.0 * t89 + t95 / 4.0);
        let t100 = t98 / 3.0 < 5e-13;
        let t104 = t86 / 6.0 - 2.0 / 3.0 * t89 + t95 / 12.0;
        let t105 = 0.0 < t104;
        let t106 = piecewise3(t105, 5e-13, -5e-13);
        let t107 = piecewise3(t100, t106, t104);
        let t108 = xc_mgga_x_br89_get_x(t107);
        let t110 = rmath::exp(t108 / 3.0);
        let t111 = 1.0 / t110;
        let t112 = rmath::exp(-t108);
        let t114 = 1.0 + t108 / 2.0;
        let t115 = t112 * t114;
        let t116 = 1.0 - t115;
        let t117 = 1.0 / t116;
        let t118 = t111 * t117;
        let t119 = t118 * t108;
        let t122 = piecewise3(t72, 0.0, t81 * t119 / 2.0);
        let t123 = t71 + t122;
        let t124 = param_cab * t123;
        let t125 = 1.0 + t124;
        let t126 = rmath::ln(t125);
        let t127 = t124 - t126;
        let t128 = t124 * t127;
        let t130 = 0.2 * t9 * t128;
        let t132 = 1.0 + t21 <= zeta_threshold;
        let t134 = 1.0 - t21 <= zeta_threshold;
        let t135 = piecewise5(t132, t16, t134, t20, t21);
        let t136 = 1.0 + t135;
        let t137 = t136 * t136;
        let t138 = pow_1_3(t136);
        let t139 = t138 * t138;
        let t141 = t11 * t11;
        let t142 = t139 * t137 * t141;
        let t143 = pow_1_3(t4);
        let t144 = t143 * t143;
        let t145 = t144 * t4;
        let t150 = 2.0 * tau0 * t34 - sigma0 * t43 / 4.0;
        let t151 = t145 * t150;
        let t152 = param_css * param_css;
        let t153 = t152 * t152;
        let t154 = t151 * t153;
        let t155 = t142 * t154;
        let t157 = 1.0 / t25 / t24;
        let t158 = t59 * t59;
        let t159 = t158 * t158;
        let t160 = 1.0 / t159;
        let t161 = t157 * t160;
        let t162 = t65 * t65;
        let t163 = t162 * t162;
        let t164 = 1.0 / t163;
        let t165 = t57 * t57;
        let t166 = t165 * t165;
        let t167 = t164 * t166;
        let t168 = param_css * t11;
        let t169 = t168 * t26;
        let t170 = t29 * t60;
        let t171 = t66 * t57;
        let t175 = 1.0 + t169 * t170 * t171 / 2.0;
        let t176 = rmath::ln(t175);
        let t177 = 1.0 / param_css;
        let t178 = t176 * t177;
        let t179 = t141 * t25;
        let t180 = t178 * t179;
        let t181 = t28 * t59;
        let t182 = 1.0 / t57;
        let t183 = t65 * t182;
        let t186 = -t180 * t181 * t183 + 1.0;
        let t188 = t161 * t167 * t186;
        let t191 = piecewise3(t10, 0.0, -0.0005433422936572482 * t155 * t188);
        let t192 = piecewise5(t134, t16, t132, t20, -t21);
        let t193 = 1.0 + t192;
        let t194 = t193 * t193;
        let t195 = pow_1_3(t193);
        let t196 = t195 * t195;
        let t198 = t196 * t194 * t141;
        let t203 = 2.0 * tau1 * t85 - sigma2 * t94 / 4.0;
        let t204 = t145 * t203;
        let t205 = t204 * t153;
        let t206 = t198 * t205;
        let t208 = 1.0 / t78 / t77;
        let t209 = t110 * t110;
        let t210 = t209 * t209;
        let t211 = 1.0 / t210;
        let t212 = t208 * t211;
        let t213 = t116 * t116;
        let t214 = t213 * t213;
        let t215 = 1.0 / t214;
        let t216 = t108 * t108;
        let t217 = t216 * t216;
        let t218 = t215 * t217;
        let t219 = t168 * t79;
        let t220 = t29 * t111;
        let t221 = t117 * t108;
        let t225 = 1.0 + t219 * t220 * t221 / 2.0;
        let t226 = rmath::ln(t225);
        let t227 = t226 * t177;
        let t228 = t141 * t78;
        let t229 = t227 * t228;
        let t230 = t28 * t110;
        let t231 = 1.0 / t108;
        let t232 = t116 * t231;
        let t235 = -t229 * t230 * t232 + 1.0;
        let t237 = t212 * t218 * t235;
        let t240 = piecewise3(t72, 0.0, -0.0005433422936572482 * t206 * t237);
        let tzk0 = -t130 + t191 + t240;
        zk[ip] += tzk0;
    }
}
