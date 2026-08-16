//! GGA_C_LM vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lm.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_lm_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_lm_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = 1.0 / M_PI;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t6 = 1.0 + t1 * t3 / 36000.0;
        let t7 = M_CBRT3;
        let t8 = t7 * t7;
        let t9 = pow_1_3(t1);
        let t10 = 1.0 / t9;
        let t11 = t8 * t10;
        let t12 = M_CBRT4;
        let t13 = pow_1_3(t2);
        let t15 = t11 * t12 * t13;
        let t17 = 1.0 + 10.0 * t15;
        let t18 = f64::ln(t17);
        let t20 = 0.252e-1 * t6 * t18;
        let t21 = t9 * t9;
        let t22 = t8 * t21;
        let t23 = t13 * t13;
        let t24 = 1.0 / t23;
        let t25 = t12 * t24;
        let t26 = t22 * t25;
        let t27 = 0.7e-5 * t26;
        let t28 = t7 * t9;
        let t29 = t12 * t12;
        let t32 = t28 * t29 / t13;
        let t33 = 0.105e-3 * t32;
        let t34 = rho0 - rho1;
        let t35 = t34 * t3;
        let t36 = 1.0 + t35;
        let t37 = t36 <= zeta_threshold;
        let t38 = pow_1_3(zeta_threshold);
        let t39 = t38 * zeta_threshold;
        let t40 = pow_1_3(t36);
        let t41 = t40 * t36;
        let t42 = piecewise3(t37, t39, t41);
        let t43 = 1.0 - t35;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(t43);
        let t46 = t45 * t43;
        let t47 = piecewise3(t44, t39, t46);
        let t49 = M_CBRT2;
        let t52 = 1.0 / (2.0 * t49 - 2.0);
        let t53 = (t42 + t47 - 2.0) * t52;
        let t55 = 1.0 + 0.56588424210451674939e-6 * t3;
        let t57 = 1.0 + 25.0 * t15;
        let t58 = f64::ln(t57);
        let t63 = -0.127e-1 * t55 * t58 - 0.64355555555555555556e-5 * t26 + 0.83833333333333333334e-4 * t32 - 0.41666666666666666667e-2 + t20;
        let t64 = t53 * t63;
        let t65 = M_PI * t8;
        let t66 = M_PI * M_PI;
        let t67 = pow_1_3(t66);
        let t69 = 1.0 / t67 / t66;
        let t70 = rho0 * rho0;
        let t71 = pow_1_3(rho0);
        let t72 = t71 * t71;
        let t74 = 1.0 / t72 / t70;
        let t75 = sigma0 * t74;
        let t77 = rho1 * rho1;
        let t78 = pow_1_3(rho1);
        let t79 = t78 * t78;
        let t81 = 1.0 / t79 / t77;
        let t82 = sigma2 * t81;
        let t87 = t38 * t38;
        let t88 = t87 * zeta_threshold;
        let t89 = t40 * t40;
        let t90 = t89 * t36;
        let t91 = piecewise3(t37, t88, t90);
        let t92 = t45 * t45;
        let t93 = t92 * t43;
        let t94 = piecewise3(t44, t88, t93);
        let t95 = t91 + t94;
        let t96 = f64::sqrt(t95);
        let t98 = M_SQRT2;
        let t99 = 1.0 / t96 * t98;
        let t100 = t7 * param_lm_f;
        let t101 = f64::powf(t1, 1.0 / 6.0);
        let t102 = 1.0 / t101;
        let t104 = sigma0 + 2.0 * sigma1 + sigma2;
        let t105 = f64::sqrt(t104);
        let t106 = t102 * t105;
        let t107 = f64::powf(t2, 1.0 / 6.0);
        let t112 = f64::exp(-t100 * t106 / t107 / t2);
        let t113 = t112 * t104;
        let t114 = t2 * t2;
        let t116 = 1.0 / t23 / t114;
        let t121 = t69 * (-7.0 / 36.0 * t49 * (t75 * t42 + t82 * t47) + 2.0 * t99 * t113 * t116);
        let t124 = t65 * t121 * t13 / 144.0;
        let tzk0 = -t20 + t27 - t33 + 0.84e-2 + t64 + t124;
        zk[ip] += tzk0;
        let t125 = 1.0 / t114;
        let t127 = t1 * t125 * t18;
        let t128 = 0.7e-6 * t127;
        let t130 = t6 * t8 * t10;
        let t131 = 1.0 / t17;
        let t133 = t130 * t25 * t131;
        let t134 = 0.84e-1 * t133;
        let t136 = 1.0 / t23 / t2;
        let t137 = t12 * t136;
        let t138 = t22 * t137;
        let t139 = 0.46666666666666666667e-5 * t138;
        let t140 = t13 * t2;
        let t142 = t29 / t140;
        let t143 = t28 * t142;
        let t144 = 0.35e-4 * t143;
        let t145 = t34 * t125;
        let t146 = t3 - t145;
        let t149 = piecewise3(t37, 0.0, 4.0 / 3.0 * t40 * t146);
        let t150 = -t146;
        let t153 = piecewise3(t44, 0.0, 4.0 / 3.0 * t45 * t150);
        let t155 = (t149 + t153) * t52;
        let t156 = t155 * t63;
        let t160 = t55 * t8 * t10;
        let t161 = 1.0 / t57;
        let t167 = 0.71867298747273627173e-8 * t125 * t58 - 0.10583333333333333333e0 * t160 * t25 * t161 + 0.42903703703703703704e-5 * t138 - 0.27944444444444444445e-4 * t143 - t128 + t134;
        let t168 = t53 * t167;
        let t171 = 1.0 / t72 / t70 / rho0;
        let t172 = sigma0 * t171;
        let t182 = 1.0 / t96 / t95 * t98;
        let t183 = t182 * t112;
        let t184 = t104 * t116;
        let t187 = piecewise3(t37, 0.0, 5.0 / 3.0 * t89 * t146);
        let t190 = piecewise3(t44, 0.0, 5.0 / 3.0 * t92 * t150);
        let t191 = t187 + t190;
        let t194 = t99 * t100;
        let t195 = t105 * t104;
        let t196 = t102 * t195;
        let t197 = t114 * t114;
        let t198 = t107 * t107;
        let t199 = t198 * t198;
        let t200 = t199 * t107;
        let t202 = 1.0 / t200 / t197;
        let t203 = t202 * t112;
        let t206 = 7.0 / 3.0 * t194 * t196 * t203;
        let t207 = t114 * t2;
        let t209 = 1.0 / t23 / t207;
        let t212 = 16.0 / 3.0 * t99 * t113 * t209;
        let t214 = t69 * (-7.0 / 36.0 * t49 * (-8.0 / 3.0 * t172 * t42 + t75 * t149 + t82 * t153) - t183 * t184 * t191 + t206 - t212);
        let t216 = t65 * t214 * t13;
        let t217 = t216 / 144.0;
        let t219 = t65 * t121 * t24;
        let t220 = t219 / 432.0;
        let tvrho0 = -t20 + t27 - t33 + 0.84e-2 + t64 + t124 + t2 * (t128 - t134 - t139 + t144 + t156 + t168 + t217 + t220);
        vrho[ip * 2] += tvrho0;
        let t223 = -t3 - t145;
        let t226 = piecewise3(t37, 0.0, 4.0 / 3.0 * t40 * t223);
        let t227 = -t223;
        let t230 = piecewise3(t44, 0.0, 4.0 / 3.0 * t45 * t227);
        let t232 = (t226 + t230) * t52;
        let t233 = t232 * t63;
        let t237 = 1.0 / t79 / t77 / rho1;
        let t238 = sigma2 * t237;
        let t247 = piecewise3(t37, 0.0, 5.0 / 3.0 * t89 * t223);
        let t250 = piecewise3(t44, 0.0, 5.0 / 3.0 * t92 * t227);
        let t251 = t247 + t250;
        let t255 = t69 * (-7.0 / 36.0 * t49 * (t75 * t226 - 8.0 / 3.0 * t238 * t47 + t82 * t230) - t183 * t184 * t251 + t206 - t212);
        let t257 = t65 * t255 * t13;
        let t258 = t257 / 144.0;
        let tvrho1 = -t20 + t27 - t33 + 0.84e-2 + t64 + t124 + t2 * (t128 - t134 - t139 + t144 + t233 + t168 + t258 + t220);
        vrho[ip * 2 + 1] += tvrho1;
        let t261 = t140 * M_PI;
        let t262 = t8 * t69;
        let t263 = t49 * t74;
        let t267 = 1.0 / t200 / t207;
        let t268 = t267 * t112;
        let t270 = t194 * t106 * t268;
        let t271 = t112 * t116;
        let t272 = t99 * t271;
        let t273 = 2.0 * t272;
        let t274 = -7.0 / 36.0 * t263 * t42 - t270 + t273;
        let tvsigma0 = t261 * t262 * t274 / 144.0;
        vsigma[ip * 3] += tvsigma0;
        let t279 = -2.0 * t270 + 4.0 * t272;
        let tvsigma1 = t261 * t262 * t279 / 144.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t282 = t49 * t81;
        let t285 = -7.0 / 36.0 * t282 * t47 - t270 + t273;
        let tvsigma2 = t261 * t262 * t285 / 144.0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
