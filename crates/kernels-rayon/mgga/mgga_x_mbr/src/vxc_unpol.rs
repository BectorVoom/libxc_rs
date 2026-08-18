//! MGGA_X_MBR vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbr.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbr_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_lambda: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = param_lambda * param_lambda;
        let t23 = t22 - param_lambda + 1.0 / 2.0;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = tau[ip] * t25;
        let t27 = t15 * t15;
        let t29 = 1.0 / t27 / rho[ip];
        let t31 = 2.0 * t26 * t29;
        let t32 = M_CBRT6;
        let t33 = t32 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = t33 * t36;
        let t39 = sigma[ip] * t25;
        let t40 = rho[ip] * rho[ip];
        let t42 = 1.0 / t27 / t40;
        let t43 = t39 * t42;
        let t49 = pow_2(2.0 * param_lambda - 1.0);
        let t50 = t49 * t32;
        let t51 = 1.0 / t36;
        let t52 = t50 * t51;
        let t55 = t49 * t49;
        let t56 = param_beta * t55;
        let t57 = t56 * t33;
        let t59 = 1.0 / t35 / t34;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t59 * t60;
        let t62 = t40 * t40;
        let t63 = t62 * rho[ip];
        let t65 = 1.0 / t15 / t63;
        let t66 = t24 * t65;
        let t70 = 1.0 + 175.0 / 162.0 * t52 * t43 + t57 * t61 * t66 / 288.0;
        let t71 = f64::powf(t70, 1.0 / 5.0);
        let t75 = t49 * sigma[ip];
        let t76 = t25 * t42;
        let t82 = t23 * (t31 - 3.0 / 5.0 * t37 - t43 / 36.0) + t37 * (t71 - 1.0) / 5.0 - param_gamma * (t31 - t75 * t76 / 4.0) / 3.0;
        let t83 = f64::abs(t82);
        let t84 = t83 < 5e-13;
        let t85 = 0.0 < t82;
        let t86 = piecewise3(t85, 5e-13, -5e-13);
        let t87 = piecewise3(t84, t86, t82);
        let t88 = xc_mgga_x_br89_get_x(t87);
        let t90 = f64::exp(t88 / 3.0);
        let t91 = t21 * t90;
        let t92 = f64::exp(-t88);
        let t94 = 1.0 + t88 / 2.0;
        let t95 = t92 * t94;
        let t96 = 1.0 - t95;
        let t97 = 1.0 / t88;
        let t98 = t96 * t97;
        let t99 = t91 * t98;
        let t102 = piecewise3(t3, 0.0, -t20 * t99 / 4.0);
        let tzk0 = 2.0 * t102;
        zk[ip] += tzk0;
        let t105 = t14 / t27 * t19;
        let t108 = M_CBRTPI;
        let t109 = t108 * t108;
        let t110 = t21 * t109;
        let t111 = piecewise3(t85, 0.0, 0.0);
        let t113 = 10.0 / 3.0 * t26 * t42;
        let t114 = t40 * rho[ip];
        let t116 = 1.0 / t27 / t114;
        let t117 = t39 * t116;
        let t121 = t71 * t71;
        let t122 = t121 * t121;
        let t123 = 1.0 / t122;
        let t126 = t62 * t40;
        let t128 = 1.0 / t15 / t126;
        let t129 = t24 * t128;
        let t133 = -700.0 / 243.0 * t52 * t117 - t57 * t61 * t129 / 54.0;
        let t137 = t25 * t116;
        let t144 = piecewise3(t84, t111, t23 * (-t113 + 2.0 / 27.0 * t117) + t37 * t123 * t133 / 25.0 - param_gamma * (-t113 + 2.0 / 3.0 * t75 * t137) / 3.0);
        let t145 = t110 * t144;
        let t146 = t20 * t145;
        let t147 = t87 * t87;
        let t148 = 1.0 / t147;
        let t150 = f64::exp(-2.0 / 3.0 * t88);
        let t151 = 1.0 / t150;
        let t152 = t148 * t151;
        let t153 = t88 * t88;
        let t155 = t153 - 2.0 * t88 + 3.0;
        let t156 = 1.0 / t155;
        let t157 = t152 * t156;
        let t158 = t88 - 2.0;
        let t159 = t158 * t158;
        let t160 = t159 * t90;
        let t161 = t160 * t98;
        let t162 = t157 * t161;
        let t165 = t109 * t144;
        let t166 = t165 * t152;
        let t167 = t156 * t159;
        let t168 = t167 * t95;
        let t170 = t165 * t148;
        let t171 = t151 * t156;
        let t172 = t159 * t92;
        let t173 = t171 * t172;
        let t176 = t166 * t168 - t170 * t173 / 2.0;
        let t177 = t176 * t97;
        let t178 = t91 * t177;
        let t181 = t91 * t96;
        let t182 = t20 * t181;
        let t183 = 1.0 / t153;
        let t184 = t183 * t109;
        let t186 = t152 * t167;
        let t187 = t184 * t144 * t186;
        let t191 = piecewise3(t3, 0.0, -t105 * t99 / 12.0 - t146 * t162 / 12.0 - t20 * t178 / 4.0 + t182 * t187 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t191 + 2.0 * t102;
        vrho[ip] += tvrho0;
        let t194 = t23 * t25;
        let t195 = t194 * t42;
        let t197 = t51 * t25;
        let t201 = t59 * sigma[ip];
        let t205 = 175.0 / 162.0 * t50 * t197 * t42 + t57 * t201 * t66 / 144.0;
        let t209 = param_gamma * t49;
        let t213 = piecewise3(t84, t111, -t195 / 36.0 + t37 * t123 * t205 / 25.0 + t209 * t76 / 12.0);
        let t214 = t110 * t213;
        let t215 = t20 * t214;
        let t218 = t109 * t213;
        let t219 = t218 * t152;
        let t221 = t218 * t148;
        let t224 = t219 * t168 - t221 * t173 / 2.0;
        let t225 = t224 * t97;
        let t226 = t91 * t225;
        let t230 = t184 * t213 * t186;
        let t234 = piecewise3(t3, 0.0, -t215 * t162 / 12.0 - t20 * t226 / 4.0 + t182 * t230 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t234;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t238 = param_gamma * t25;
        let t242 = piecewise3(t84, t111, 2.0 * t194 * t29 - 2.0 / 3.0 * t238 * t29);
        let t243 = t110 * t242;
        let t244 = t20 * t243;
        let t247 = t109 * t242;
        let t248 = t247 * t152;
        let t250 = t247 * t148;
        let t253 = t248 * t168 - t250 * t173 / 2.0;
        let t254 = t253 * t97;
        let t255 = t91 * t254;
        let t259 = t184 * t242 * t186;
        let t263 = piecewise3(t3, 0.0, -t244 * t162 / 12.0 - t20 * t255 / 4.0 + t182 * t259 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t263;
        vtau[ip] += tvtau0;
    }
}
