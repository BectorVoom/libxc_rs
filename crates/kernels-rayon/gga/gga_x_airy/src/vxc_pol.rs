//! GGA_X_AIRY vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
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
        let t40 = f64::powf(t39, 0.2626712e1);
        let t42 = 1.0 + 0.13471619689594796103e-3 * t40;
        let t43 = f64::powf(t42, -0.657946e0);
        let t46 = f64::powf(t39, 0.3217063e1);
        let t48 = f64::powf(t39, 0.3223476e1);
        let t50 = 1.0 - 0.45212413010769857073e-1 * t46 + 0.45402221956620378581e-1 * t48;
        let t51 = f64::powf(t39, 0.3473804e1);
        let t53 = 1.0 + 0.47702180224903349918e-3 * t51;
        let t54 = 1.0 / t53;
        let t56 = 0.60146019220211109872e-4 * t40 * t43 + t50 * t54;
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
        let t77 = f64::powf(t76, 0.2626712e1);
        let t79 = 1.0 + 0.13471619689594796103e-3 * t77;
        let t80 = f64::powf(t79, -0.657946e0);
        let t83 = f64::powf(t76, 0.3217063e1);
        let t85 = f64::powf(t76, 0.3223476e1);
        let t87 = 1.0 - 0.45212413010769857073e-1 * t83 + 0.45402221956620378581e-1 * t85;
        let t88 = f64::powf(t76, 0.3473804e1);
        let t90 = 1.0 + 0.47702180224903349918e-3 * t88;
        let t91 = 1.0 / t90;
        let t93 = 0.60146019220211109872e-4 * t77 * t80 + t87 * t91;
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
        let t116 = f64::powf(t39, 0.1626712e1);
        let t118 = t116 * t43 * t29;
        let t119 = t32 * t34;
        let t120 = rho0 * rho0;
        let t122 = 1.0 / t35 / t120;
        let t123 = t119 * t122;
        let t126 = f64::powf(t39, 0.4253424e1);
        let t127 = f64::powf(t42, -0.1657946e1);
        let t129 = t126 * t127 * t29;
        let t132 = f64::powf(t39, 0.2217063e1);
        let t133 = t132 * t29;
        let t136 = f64::powf(t39, 0.2223476e1);
        let t137 = t136 * t29;
        let t140 = 0.19393490805022174494e0 * t133 * t123 - 0.19513729709845177529e0 * t137 * t123;
        let t142 = t53 * t53;
        let t143 = 1.0 / t142;
        let t144 = t50 * t143;
        let t145 = f64::powf(t39, 0.2473804e1);
        let t146 = t144 * t145;
        let t148 = t33 * t34 * t122;
        let t151 = -0.21064836058394555311e-3 * t118 * t123 + 0.18671024483029835192e-7 * t129 * t123 + t140 * t54 + 0.22094403263198687541e-2 * t146 * t148;
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
        let t195 = f64::powf(t76, 0.1626712e1);
        let t197 = t195 * t80 * t29;
        let t198 = t32 * t71;
        let t199 = rho1 * rho1;
        let t201 = 1.0 / t72 / t199;
        let t202 = t198 * t201;
        let t205 = f64::powf(t76, 0.4253424e1);
        let t206 = f64::powf(t79, -0.1657946e1);
        let t208 = t205 * t206 * t29;
        let t211 = f64::powf(t76, 0.2217063e1);
        let t212 = t211 * t29;
        let t215 = f64::powf(t76, 0.2223476e1);
        let t216 = t215 * t29;
        let t219 = 0.19393490805022174494e0 * t212 * t202 - 0.19513729709845177529e0 * t216 * t202;
        let t221 = t90 * t90;
        let t222 = 1.0 / t221;
        let t223 = t87 * t222;
        let t224 = f64::powf(t76, 0.2473804e1);
        let t225 = t223 * t224;
        let t227 = t33 * t71 * t201;
        let t230 = -0.21064836058394555311e-3 * t197 * t202 + 0.18671024483029835192e-7 * t208 * t202 + t219 * t91 + 0.22094403263198687541e-2 * t225 * t227;
        let t235 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t191 * t93 - t170 - 3.0 / 8.0 * t5 * t70 * t230);
        let tvrho1 = t60 + t97 + t6 * (t185 + t235);
        vrho[ip * 2 + 1] += tvrho1;
        let t238 = 1.0 / t34;
        let t239 = t32 * t238;
        let t240 = t239 * t37;
        let t249 = -0.72725590518833154352e-1 * t133 * t240 + 0.73176486411919415733e-1 * t137 * t240;
        let t252 = t33 * t238 * t37;
        let t255 = 0.78993135218979582417e-4 * t118 * t240 - 0.7001634181136188197e-8 * t129 * t240 + t249 * t54 - 0.82854012236995078279e-3 * t146 * t252;
        let t259 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t255);
        let tvsigma0 = t6 * t259;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t260 = 1.0 / t71;
        let t261 = t32 * t260;
        let t262 = t261 * t74;
        let t271 = -0.72725590518833154352e-1 * t212 * t262 + 0.73176486411919415733e-1 * t216 * t262;
        let t274 = t33 * t260 * t74;
        let t277 = 0.78993135218979582417e-4 * t197 * t262 - 0.7001634181136188197e-8 * t208 * t262 + t271 * t91 - 0.82854012236995078279e-3 * t225 * t274;
        let t281 = piecewise3(t61, 0.0, -3.0 / 8.0 * t5 * t70 * t277);
        let tvsigma2 = t6 * t281;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
