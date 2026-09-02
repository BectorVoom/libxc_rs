//! GGA_X_CHACHIYO vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_chachiyo.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_chachiyo_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t19 = t18 + 1.0;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = t3 * t3;
        let t29 = t2 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * sigma0;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t40 = M_PI * M_PI;
        let t41 = t2 * t2;
        let t42 = t41 * t3;
        let t43 = t30 * t30;
        let t44 = rmath::sqrt(sigma0);
        let t47 = 1.0 / t33 / rho0;
        let t49 = t42 * t43 * t44 * t47;
        let t51 = t49 / 27.0 + 1.0;
        let t52 = rmath::ln(t51);
        let t54 = 2.0 / 81.0 * t29 * t31 * t36 + t40 * t52;
        let t57 = t49 / 9.0 + t40;
        let t58 = 1.0 / t57;
        let t59 = 1.0 / t52;
        let t60 = t58 * t59;
        let t61 = t27 * t54 * t60;
        let t64 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t61);
        let t65 = rho1 <= dens_threshold;
        let t66 = -t16;
        let t68 = piecewise5(t14, t11, t10, t15, t66 * t7);
        let t69 = t68 + 1.0;
        let t70 = t69 <= zeta_threshold;
        let t71 = pow_1_3(t69);
        let t73 = piecewise3(t70, t22, t71 * t69);
        let t74 = t5 * t73;
        let t75 = t30 * sigma2;
        let t76 = rho1 * rho1;
        let t77 = pow_1_3(rho1);
        let t78 = t77 * t77;
        let t80 = 1.0 / t78 / t76;
        let t84 = rmath::sqrt(sigma2);
        let t87 = 1.0 / t77 / rho1;
        let t89 = t42 * t43 * t84 * t87;
        let t91 = t89 / 27.0 + 1.0;
        let t92 = rmath::ln(t91);
        let t94 = 2.0 / 81.0 * t29 * t75 * t80 + t40 * t92;
        let t97 = t89 / 9.0 + t40;
        let t98 = 1.0 / t97;
        let t99 = 1.0 / t92;
        let t100 = t98 * t99;
        let t101 = t27 * t94 * t100;
        let t104 = piecewise3(t65, 0.0, -3.0 / 8.0 * t74 * t101);
        let tzk0 = t64 + t104;
        zk[ip] += tzk0;
        let t105 = t6 * t6;
        let t106 = 1.0 / t105;
        let t107 = t16 * t106;
        let t109 = piecewise5(t10, 0.0, t14, 0.0, t7 - t107);
        let t112 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t109);
        let t113 = t5 * t112;
        let t116 = t27 * t27;
        let t117 = 1.0 / t116;
        let t119 = t117 * t54 * t60;
        let t121 = t26 * t119 / 8.0;
        let t122 = t32 * rho0;
        let t124 = 1.0 / t34 / t122;
        let t130 = t3 * t40 * t41 * t43;
        let t132 = 1.0 / t33 / t32;
        let t133 = t44 * t132;
        let t134 = 1.0 / t51;
        let t135 = t133 * t134;
        let t138 = -16.0 / 243.0 * t29 * t31 * t124 - 4.0 / 81.0 * t130 * t135;
        let t140 = t27 * t138 * t60;
        let t143 = t25 * t27;
        let t144 = t57 * t57;
        let t145 = 1.0 / t144;
        let t146 = t54 * t145;
        let t147 = t143 * t146;
        let t148 = t59 * t43;
        let t149 = t148 * t133;
        let t152 = t54 * t58;
        let t153 = t143 * t152;
        let t154 = t52 * t52;
        let t155 = 1.0 / t154;
        let t156 = t155 * t43;
        let t157 = t156 * t135;
        let t161 = piecewise3(t1, 0.0, -3.0 / 8.0 * t113 * t61 - t121 - 3.0 / 8.0 * t26 * t140 - t147 * t149 / 6.0 - t153 * t157 / 18.0);
        let t162 = t66 * t106;
        let t164 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t162);
        let t167 = piecewise3(t70, 0.0, 4.0 / 3.0 * t71 * t164);
        let t168 = t5 * t167;
        let t172 = t117 * t94 * t100;
        let t174 = t74 * t172 / 8.0;
        let t176 = piecewise3(t65, 0.0, -3.0 / 8.0 * t168 * t101 - t174);
        let tvrho0 = t64 + t104 + t6 * (t161 + t176);
        vrho[ip * 2] += tvrho0;
        let t180 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t107);
        let t183 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t180);
        let t184 = t5 * t183;
        let t188 = piecewise3(t1, 0.0, -3.0 / 8.0 * t184 * t61 - t121);
        let t190 = piecewise5(t14, 0.0, t10, 0.0, t7 - t162);
        let t193 = piecewise3(t70, 0.0, 4.0 / 3.0 * t71 * t190);
        let t194 = t5 * t193;
        let t197 = t76 * rho1;
        let t199 = 1.0 / t78 / t197;
        let t204 = 1.0 / t77 / t76;
        let t205 = t84 * t204;
        let t206 = 1.0 / t91;
        let t207 = t205 * t206;
        let t210 = -16.0 / 243.0 * t29 * t75 * t199 - 4.0 / 81.0 * t130 * t207;
        let t212 = t27 * t210 * t100;
        let t215 = t73 * t27;
        let t216 = t97 * t97;
        let t217 = 1.0 / t216;
        let t218 = t94 * t217;
        let t219 = t215 * t218;
        let t220 = t99 * t43;
        let t221 = t220 * t205;
        let t224 = t94 * t98;
        let t225 = t215 * t224;
        let t226 = t92 * t92;
        let t227 = 1.0 / t226;
        let t228 = t227 * t43;
        let t229 = t228 * t207;
        let t233 = piecewise3(t65, 0.0, -3.0 / 8.0 * t194 * t101 - t174 - 3.0 / 8.0 * t74 * t212 - t219 * t221 / 6.0 - t225 * t229 / 18.0);
        let tvrho1 = t64 + t104 + t6 * (t188 + t233);
        vrho[ip * 2 + 1] += tvrho1;
        let t239 = 1.0 / t44;
        let t240 = t239 * t47;
        let t241 = t240 * t134;
        let t244 = 2.0 / 81.0 * t29 * t30 * t36 + t130 * t241 / 54.0;
        let t246 = t27 * t244 * t60;
        let t249 = t148 * t240;
        let t252 = t156 * t241;
        let t256 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t246 + t147 * t249 / 16.0 + t153 * t252 / 48.0);
        let tvsigma0 = t6 * t256;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t260 = 1.0 / t84;
        let t261 = t260 * t87;
        let t262 = t261 * t206;
        let t265 = 2.0 / 81.0 * t29 * t30 * t80 + t130 * t262 / 54.0;
        let t267 = t27 * t265 * t100;
        let t270 = t220 * t261;
        let t273 = t228 * t262;
        let t277 = piecewise3(t65, 0.0, -3.0 / 8.0 * t74 * t267 + t219 * t270 / 16.0 + t225 * t273 / 48.0);
        let tvsigma2 = t6 * t277;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
