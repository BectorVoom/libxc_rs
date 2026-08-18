//! GGA_X_AIRY vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_airy_vxc_pol(
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
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = f64::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t39 = t33 * t34 * t37;
        let t40 = f64::powf(t39, 2.626712);
        let t42 = 1.0 + 0.00013471619689594795 * t40;
        let t43 = f64::powf(t42, -0.657946);
        let t46 = f64::powf(t39, 3.217063);
        let t48 = f64::powf(t39, 3.223476);
        let t50 = 1.0 - 0.04521241301076986 * t46 + 0.04540222195662038 * t48;
        let t51 = f64::powf(t39, 3.473804);
        let t53 = 1.0 + 0.0004770218022490335 * t51;
        let t54 = 1.0 / t53;
        let t56 = 6.014601922021111e-05 * t40 * t43 + t50 * t54;
        let t60 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t56);
        let t61 = rho1 <= dens_threshold;
        let t62 = -t16;
        let t64 = piecewise5(t14, t11, t10, t15, t62 * t7);
        let t65 = 1.0 + t64;
        let t66 = t65 <= zeta_threshold;
        let t67 = pow_1_3(t65);
        let t69 = piecewise3(t66, t22, t67 * t65);
        let t70 = t69 * t26;
        let t71 = f64::sqrt(sigma2);
        let t72 = pow_1_3(rho1);
        let t74 = 1.0 / t72 / rho1;
        let t76 = t33 * t71 * t74;
        let t77 = f64::powf(t76, 2.626712);
        let t79 = 1.0 + 0.00013471619689594795 * t77;
        let t80 = f64::powf(t79, -0.657946);
        let t83 = f64::powf(t76, 3.217063);
        let t85 = f64::powf(t76, 3.223476);
        let t87 = 1.0 - 0.04521241301076986 * t83 + 0.04540222195662038 * t85;
        let t88 = f64::powf(t76, 3.473804);
        let t90 = 1.0 + 0.0004770218022490335 * t88;
        let t91 = 1.0 / t90;
        let t93 = 6.014601922021111e-05 * t77 * t80 + t87 * t91;
        let t97 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t70 * t93);
        let tzk0 = t60 + t97;
        zk[ip] += tzk0;
        let t98 = t6 * t6;
        let t99 = 1.0 / t98;
        let t100 = t16 * t99;
        let t102 = piecewise5(t10, 0.0, t14, 0.0, t7 - t100);
        let t105 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t102);
        let t106 = t105 * t26;
        let t110 = t26 * t26;
        let t111 = 1.0 / t110;
        let t112 = t25 * t111;
        let t115 = t5 * t112 * t56 / 8.0;
        let t116 = f64::powf(t39, 1.626712);
        let t118 = t116 * t43 * t29;
        let t119 = t32 * t34;
        let t120 = rho0 * rho0;
        let t122 = 1.0 / t35 / t120;
        let t123 = t119 * t122;
        let t126 = f64::powf(t39, 4.253424);
        let t127 = f64::powf(t42, -1.657946);
        let t129 = t126 * t127 * t29;
        let t132 = f64::powf(t39, 2.217063);
        let t133 = t132 * t29;
        let t136 = f64::powf(t39, 2.223476);
        let t137 = t136 * t29;
        let t140 = 0.19393490805022173 * t133 * t123 - 0.19513729709845176 * t137 * t123;
        let t142 = t53 * t53;
        let t143 = 1.0 / t142;
        let t144 = t50 * t143;
        let t145 = f64::powf(t39, 2.473804);
        let t146 = t144 * t145;
        let t148 = t33 * t34 * t122;
        let t151 = -0.00021064836058394556 * t118 * t123 + 1.8671024483029836e-08 * t129 * t123 + t140 * t54 + 0.0022094403263198687 * t146 * t148;
        let t156 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t106 * t56 - t115 - 3.0 / 8.0 * t5 * t27 * t151);
        let t157 = t62 * t99;
        let t159 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t157);
        let t162 = piecewise3(t66, 0.0, 4.0 / 3.0 * t67 * t159);
        let t163 = t162 * t26;
        let t167 = t69 * t111;
        let t170 = t5 * t167 * t93 / 8.0;
        let t172 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t163 * t93 - t170);
        let tvrho0 = t60 + t97 + t6 * (t156 + t172);
        vrho[ip * 2] += tvrho0;
        let t176 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t100);
        let t179 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t176);
        let t180 = t179 * t26;
        let t185 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t180 * t56 - t115);
        let t187 = piecewise5(t14, 0.0, t10, 0.0, t7 - t157);
        let t190 = piecewise3(t66, 0.0, 4.0 / 3.0 * t67 * t187);
        let t191 = t190 * t26;
        let t195 = f64::powf(t76, 1.626712);
        let t197 = t195 * t80 * t29;
        let t198 = t32 * t71;
        let t199 = rho1 * rho1;
        let t201 = 1.0 / t72 / t199;
        let t202 = t198 * t201;
        let t205 = f64::powf(t76, 4.253424);
        let t206 = f64::powf(t79, -1.657946);
        let t208 = t205 * t206 * t29;
        let t211 = f64::powf(t76, 2.217063);
        let t212 = t211 * t29;
        let t215 = f64::powf(t76, 2.223476);
        let t216 = t215 * t29;
        let t219 = 0.19393490805022173 * t212 * t202 - 0.19513729709845176 * t216 * t202;
        let t221 = t90 * t90;
        let t222 = 1.0 / t221;
        let t223 = t87 * t222;
        let t224 = f64::powf(t76, 2.473804);
        let t225 = t223 * t224;
        let t227 = t33 * t71 * t201;
        let t230 = -0.00021064836058394556 * t197 * t202 + 1.8671024483029836e-08 * t208 * t202 + t219 * t91 + 0.0022094403263198687 * t225 * t227;
        let t235 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t191 * t93 - t170 - 3.0 / 8.0 * t5 * t70 * t230);
        let tvrho1 = t60 + t97 + t6 * (t185 + t235);
        vrho[ip * 2 + 1] += tvrho1;
        let t238 = 1.0 / t34;
        let t239 = t32 * t238;
        let t240 = t239 * t37;
        let t249 = -0.07272559051883315 * t133 * t240 + 0.07317648641191941 * t137 * t240;
        let t252 = t33 * t238 * t37;
        let t255 = 7.899313521897959e-05 * t118 * t240 - 7.001634181136188e-09 * t129 * t240 + t249 * t54 - 0.0008285401223699508 * t146 * t252;
        let t259 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t255);
        let tvsigma0 = t6 * t259;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t260 = 1.0 / t71;
        let t261 = t32 * t260;
        let t262 = t261 * t74;
        let t271 = -0.07272559051883315 * t212 * t262 + 0.07317648641191941 * t216 * t262;
        let t274 = t33 * t260 * t74;
        let t277 = 7.899313521897959e-05 * t197 * t262 - 7.001634181136188e-09 * t208 * t262 + t271 * t91 - 0.0008285401223699508 * t225 * t274;
        let t281 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t70 * t277);
        let tvsigma2 = t6 * t281;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
