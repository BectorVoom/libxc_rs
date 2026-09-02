//! GGA_X_CAP vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_cap.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_cap_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alphaoAx: f64,
    param_c: f64,
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
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = 1.0 / t32;
        let t34 = param_alphaoAx * t29 * t33;
        let t35 = rmath::sqrt(sigma0);
        let t36 = pow_1_3(rho0);
        let t38 = 1.0 / t36 / rho0;
        let t39 = t35 * t38;
        let t40 = t29 * t33;
        let t43 = 1.0 + t40 * t39 / 12.0;
        let t44 = rmath::ln(t43);
        let t46 = param_c * t44 + 1.0;
        let t47 = 1.0 / t46;
        let t48 = t44 * t47;
        let t52 = 1.0 - t34 * t39 * t48 / 12.0;
        let t56 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t16;
        let t60 = piecewise5(t14, t11, t10, t15, t58 * t7);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t65 = piecewise3(t62, t22, t63 * t61);
        let t66 = t65 * t26;
        let t67 = rmath::sqrt(sigma2);
        let t68 = pow_1_3(rho1);
        let t70 = 1.0 / t68 / rho1;
        let t71 = t67 * t70;
        let t74 = 1.0 + t40 * t71 / 12.0;
        let t75 = rmath::ln(t74);
        let t77 = param_c * t75 + 1.0;
        let t78 = 1.0 / t77;
        let t79 = t75 * t78;
        let t83 = 1.0 - t34 * t71 * t79 / 12.0;
        let t87 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t66 * t83);
        let tzk0 = t56 + t87;
        zk[ip] += tzk0;
        let t88 = t6 * t6;
        let t89 = 1.0 / t88;
        let t90 = t16 * t89;
        let t92 = piecewise5(t10, 0.0, t14, 0.0, t7 - t90);
        let t95 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t92);
        let t96 = t95 * t26;
        let t100 = t26 * t26;
        let t101 = 1.0 / t100;
        let t102 = t25 * t101;
        let t105 = t5 * t102 * t52 / 8.0;
        let t106 = rho0 * rho0;
        let t108 = 1.0 / t36 / t106;
        let t113 = param_alphaoAx * t28;
        let t114 = t32 * t32;
        let t115 = 1.0 / t114;
        let t116 = t113 * t115;
        let t117 = t106 * rho0;
        let t118 = t36 * t36;
        let t120 = 1.0 / t118 / t117;
        let t122 = 1.0 / t43;
        let t123 = t122 * t47;
        let t128 = t113 * t115 * sigma0;
        let t130 = t46 * t46;
        let t131 = 1.0 / t130;
        let t132 = t131 * param_c;
        let t133 = t132 * t122;
        let t134 = t120 * t44 * t133;
        let t137 = t34 * t35 * t108 * t48 / 9.0 + t116 * sigma0 * t120 * t123 / 18.0 - t128 * t134 / 18.0;
        let t142 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t96 * t52 - t105 - 3.0 / 8.0 * t5 * t27 * t137);
        let t143 = t58 * t89;
        let t145 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t143);
        let t148 = piecewise3(t62, 0.0, 4.0 / 3.0 * t63 * t145);
        let t149 = t148 * t26;
        let t153 = t65 * t101;
        let t156 = t5 * t153 * t83 / 8.0;
        let t158 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t149 * t83 - t156);
        let tvrho0 = t56 + t87 + t6 * (t142 + t158);
        vrho[ip * 2] += tvrho0;
        let t162 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t90);
        let t165 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t162);
        let t166 = t165 * t26;
        let t171 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t166 * t52 - t105);
        let t173 = piecewise5(t14, 0.0, t10, 0.0, t7 - t143);
        let t176 = piecewise3(t62, 0.0, 4.0 / 3.0 * t63 * t173);
        let t177 = t176 * t26;
        let t181 = rho1 * rho1;
        let t183 = 1.0 / t68 / t181;
        let t188 = t181 * rho1;
        let t189 = t68 * t68;
        let t191 = 1.0 / t189 / t188;
        let t193 = 1.0 / t74;
        let t194 = t193 * t78;
        let t199 = t113 * t115 * sigma2;
        let t201 = t77 * t77;
        let t202 = 1.0 / t201;
        let t203 = t202 * param_c;
        let t204 = t203 * t193;
        let t205 = t191 * t75 * t204;
        let t208 = t34 * t67 * t183 * t79 / 9.0 + t116 * sigma2 * t191 * t194 / 18.0 - t199 * t205 / 18.0;
        let t213 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t177 * t83 - t156 - 3.0 / 8.0 * t5 * t66 * t208);
        let tvrho1 = t56 + t87 + t6 * (t171 + t213);
        vrho[ip * 2 + 1] += tvrho1;
        let t216 = 1.0 / t35;
        let t222 = 1.0 / t118 / t106;
        let t229 = t44 * t131;
        let t231 = t229 * param_c * t122;
        let t234 = -t34 * t216 * t38 * t48 / 24.0 - t116 * t222 * t122 * t47 / 48.0 + t113 * t115 * t222 * t231 / 48.0;
        let t238 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t234);
        let tvsigma0 = t6 * t238;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t239 = 1.0 / t67;
        let t245 = 1.0 / t189 / t181;
        let t252 = t75 * t202;
        let t254 = t252 * param_c * t193;
        let t257 = -t34 * t239 * t70 * t79 / 24.0 - t116 * t245 * t193 * t78 / 48.0 + t113 * t115 * t245 * t254 / 48.0;
        let t261 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t66 * t257);
        let tvsigma2 = t6 * t261;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
