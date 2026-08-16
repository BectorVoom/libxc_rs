//! GGA_X_SSB_SW vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ssb_sw_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_B * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t41 = param_C * t28;
        let t42 = t33 * sigma0;
        let t46 = 1.0 + t41 * t42 * t39 / 24.0;
        let t47 = 1.0 / t46;
        let t51 = param_D * t28;
        let t52 = t51 * t33;
        let t53 = t28 * t28;
        let t54 = param_E * t53;
        let t56 = 1.0 / t31 / t30;
        let t57 = sigma0 * sigma0;
        let t59 = t35 * t35;
        let t60 = t59 * rho0;
        let t62 = 1.0 / t36 / t60;
        let t66 = 1.0 + t54 * t56 * t57 * t62 / 576.0;
        let t67 = 1.0 / t66;
        let t71 = param_A + t34 * t40 * t47 / 24.0 - t52 * t40 * t67 / 24.0;
        let t75 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t71);
        let t76 = rho1 <= dens_threshold;
        let t77 = -t16;
        let t79 = piecewise5(t14, t11, t10, t15, t77 * t7);
        let t80 = 1.0 + t79;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t22, t82 * t80);
        let t85 = t84 * t26;
        let t86 = rho1 * rho1;
        let t87 = pow_1_3(rho1);
        let t88 = t87 * t87;
        let t90 = 1.0 / t88 / t86;
        let t91 = sigma2 * t90;
        let t92 = t33 * sigma2;
        let t96 = 1.0 + t41 * t92 * t90 / 24.0;
        let t97 = 1.0 / t96;
        let t101 = sigma2 * sigma2;
        let t103 = t86 * t86;
        let t104 = t103 * rho1;
        let t106 = 1.0 / t87 / t104;
        let t110 = 1.0 + t54 * t56 * t101 * t106 / 576.0;
        let t111 = 1.0 / t110;
        let t115 = param_A + t34 * t91 * t97 / 24.0 - t52 * t91 * t111 / 24.0;
        let t119 = piecewise3(t76, 0.0, -3.0 / 8.0 * t5 * t85 * t115);
        let tzk0 = t75 + t119;
        zk[ip] += tzk0;
        let t120 = t6 * t6;
        let t121 = 1.0 / t120;
        let t122 = t16 * t121;
        let t124 = piecewise5(t10, 0.0, t14, 0.0, t7 - t122);
        let t127 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t124);
        let t128 = t127 * t26;
        let t132 = t26 * t26;
        let t133 = 1.0 / t132;
        let t134 = t25 * t133;
        let t137 = t5 * t134 * t71 / 8.0;
        let t138 = t35 * rho0;
        let t140 = 1.0 / t37 / t138;
        let t141 = sigma0 * t140;
        let t146 = param_B * t53 * t56;
        let t147 = t59 * t35;
        let t149 = 1.0 / t36 / t147;
        let t151 = t46 * t46;
        let t152 = 1.0 / t151;
        let t153 = t152 * param_C;
        let t160 = t30 * t30;
        let t161 = 1.0 / t160;
        let t162 = param_D * t161;
        let t163 = t57 * sigma0;
        let t164 = t162 * t163;
        let t165 = t59 * t59;
        let t166 = t165 * rho0;
        let t167 = 1.0 / t166;
        let t168 = t66 * t66;
        let t169 = 1.0 / t168;
        let t171 = t167 * t169 * param_E;
        let t174 = -t34 * t141 * t47 / 9.0 + t146 * t57 * t149 * t153 / 216.0 + t52 * t141 * t67 / 9.0 - t164 * t171 / 432.0;
        let t179 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t128 * t71 - t137 - 3.0 / 8.0 * t5 * t27 * t174);
        let t180 = t77 * t121;
        let t182 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t180);
        let t185 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t182);
        let t186 = t185 * t26;
        let t190 = t84 * t133;
        let t193 = t5 * t190 * t115 / 8.0;
        let t195 = piecewise3(t76, 0.0, -3.0 / 8.0 * t5 * t186 * t115 - t193);
        let tvrho0 = t75 + t119 + t6 * (t179 + t195);
        vrho[ip * 2] += tvrho0;
        let t199 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t122);
        let t202 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t199);
        let t203 = t202 * t26;
        let t208 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t203 * t71 - t137);
        let t210 = piecewise5(t14, 0.0, t10, 0.0, t7 - t180);
        let t213 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t210);
        let t214 = t213 * t26;
        let t218 = t86 * rho1;
        let t220 = 1.0 / t88 / t218;
        let t221 = sigma2 * t220;
        let t225 = t103 * t86;
        let t227 = 1.0 / t87 / t225;
        let t229 = t96 * t96;
        let t230 = 1.0 / t229;
        let t231 = t230 * param_C;
        let t238 = t101 * sigma2;
        let t239 = t162 * t238;
        let t240 = t103 * t103;
        let t241 = t240 * rho1;
        let t242 = 1.0 / t241;
        let t243 = t110 * t110;
        let t244 = 1.0 / t243;
        let t246 = t242 * t244 * param_E;
        let t249 = -t34 * t221 * t97 / 9.0 + t146 * t101 * t227 * t231 / 216.0 + t52 * t221 * t111 / 9.0 - t239 * t246 / 432.0;
        let t254 = piecewise3(t76, 0.0, -3.0 / 8.0 * t5 * t214 * t115 - t193 - 3.0 / 8.0 * t5 * t85 * t249);
        let tvrho1 = t75 + t119 + t6 * (t208 + t254);
        vrho[ip * 2 + 1] += tvrho1;
        let t257 = t33 * t39;
        let t269 = 1.0 / t165;
        let t271 = t269 * t169 * param_E;
        let t274 = t29 * t257 * t47 / 24.0 - t146 * sigma0 * t62 * t153 / 576.0 - t51 * t257 * t67 / 24.0 + t162 * t57 * t271 / 1152.0;
        let t278 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t274);
        let tvsigma0 = t6 * t278;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t279 = t33 * t90;
        let t291 = 1.0 / t240;
        let t293 = t291 * t244 * param_E;
        let t296 = t29 * t279 * t97 / 24.0 - t146 * sigma2 * t106 * t231 / 576.0 - t51 * t279 * t111 / 24.0 + t162 * t101 * t293 / 1152.0;
        let t300 = piecewise3(t76, 0.0, -3.0 / 8.0 * t5 * t85 * t296);
        let tvsigma2 = t6 * t300;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
