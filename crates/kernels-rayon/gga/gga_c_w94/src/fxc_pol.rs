//! GGA_C_W94 fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_w94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_w94_fxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 0.0 < t4;
        let t6 = piecewise3(t5, t4, -t4);
        let t7 = 1e-10 < t6;
        let t8 = piecewise3(t7, t6, 1e-10);
        let t9 = pow_1_3(t8);
        let t10 = t9 * t9;
        let t12 = -t10 * t8 + 1.0;
        let t13 = rmath::sqrt(t12);
        let t15 = sigma0 + 2.0 * sigma1 + sigma2;
        let t16 = rmath::sqrt(t15);
        let t17 = t16 * t15;
        let t18 = t2 * t2;
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t22 = pow_1_3(t2);
        let t24 = 1.0 / t22 / t2;
        let t25 = t16 * t24;
        let t26 = rmath::pow(t25, 1.0 / 16.0);
        let t27 = t26 * t26;
        let t28 = t27 * t26;
        let t31 = t18 * t2;
        let t32 = 1.0 / t31;
        let t35 = M_CBRT3;
        let t37 = pow_1_3(1.0 / M_PI);
        let t38 = t35 * t37;
        let t39 = M_CBRT4;
        let t40 = t39 * t39;
        let t45 = 11.8 + 0.15067 * t28 * t17 * t20 + 0.01102 * t15 * t32 + t38 * t40 / t22 / 4.0;
        let t46 = 1.0 / t45;
        let tzk0 = -t13 * t46;
        zk[ip] += tzk0;
        let t48 = 1.0 / t13;
        let t49 = t2 * t48;
        let t50 = t46 * t10;
        let t51 = 1.0 / t18;
        let t52 = t1 * t51;
        let t53 = t3 - t52;
        let t55 = piecewise3(t5, t53, -t53);
        let t56 = piecewise3(t7, t55, 0.0);
        let t60 = t2 * t13;
        let t61 = t45 * t45;
        let t62 = 1.0 / t61;
        let t63 = t22 * t22;
        let t65 = 1.0 / t63 / t18;
        let t67 = t28 * t15 * t65;
        let t68 = t67 * t16;
        let t70 = 1.0 / t22 / t18;
        let t78 = -0.6403475 * t68 * t70 - 0.03306 * t15 * t20 - t38 * t40 * t24 / 12.0;
        let t80 = t60 * t62 * t78;
        let tvrho0 = tzk0 + 5.0 / 6.0 * t49 * t50 * t56 + t80;
        vrho[ip * 2] += tvrho0;
        let t81 = -t3 - t52;
        let t83 = piecewise3(t5, t81, -t81);
        let t84 = piecewise3(t7, t83, 0.0);
        let tvrho1 = tzk0 + 5.0 / 6.0 * t49 * t50 * t84 + t80;
        vrho[ip * 2 + 1] += tvrho1;
        let t88 = 1.0 / t16;
        let t89 = t67 * t88;
        let t90 = t89 * t24;
        let t93 = 0.2401303125 * t90 + 0.01102 * t32;
        let tvsigma0 = t60 * t62 * t93;
        vsigma[ip * 3] += tvsigma0;
        let t97 = 0.480260625 * t90 + 0.02204 * t32;
        let tvsigma1 = t60 * t62 * t97;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t99 = t48 * t46;
        let t100 = t10 * t56;
        let t101 = t99 * t100;
        let t103 = t13 * t62;
        let t105 = 2.0 * t103 * t78;
        let t107 = 1.0 / t13 / t12;
        let t108 = t2 * t107;
        let t109 = t9 * t8;
        let t110 = t46 * t109;
        let t111 = t56 * t56;
        let t115 = t49 * t62;
        let t116 = t100 * t78;
        let t117 = t115 * t116;
        let t119 = 1.0 / t9;
        let t120 = t46 * t119;
        let t124 = t1 * t32;
        let t126 = -2.0 * t51 + 2.0 * t124;
        let t128 = piecewise3(t5, t126, -t126);
        let t129 = piecewise3(t7, t128, 0.0);
        let t134 = 1.0 / t61 / t45;
        let t135 = t78 * t78;
        let t138 = 2.0 * t60 * t134 * t135;
        let t139 = t28 * t25;
        let t140 = t139 * t15;
        let t142 = 1.0 / t63 / t19;
        let t146 = 1.0 / t22 / t31;
        let t149 = t19 * t2;
        let t150 = 1.0 / t149;
        let t156 = 1.8676802083333333 * t140 * t142 + 1.4941441666666666 * t68 * t146 + 0.13224 * t15 * t150 + t38 * t40 * t70 / 9.0;
        let t158 = t60 * t62 * t156;
        let tv2rho20 = 5.0 / 3.0 * t101 + t105 + 25.0 / 36.0 * t108 * t110 * t111 - 5.0 / 3.0 * t117 + 5.0 / 9.0 * t49 * t120 * t111 + 5.0 / 6.0 * t49 * t50 * t129 - t138 + t158;
        v2rho2[ip * 3] += tv2rho20;
        let t160 = t10 * t84;
        let t161 = t99 * t160;
        let t163 = t108 * t46;
        let t164 = t109 * t84;
        let t165 = t164 * t56;
        let t168 = t160 * t78;
        let t169 = t115 * t168;
        let t171 = t49 * t46;
        let t172 = t119 * t84;
        let t173 = t172 * t56;
        let t176 = 2.0 * t124;
        let t177 = piecewise3(t5, t176, -t176);
        let t178 = piecewise3(t7, t177, 0.0);
        let tv2rho21 = 5.0 / 6.0 * t101 + t105 + 5.0 / 6.0 * t161 + 25.0 / 36.0 * t163 * t165 - 5.0 / 6.0 * t169 + 5.0 / 9.0 * t171 * t173 + 5.0 / 6.0 * t49 * t50 * t178 - 5.0 / 6.0 * t117 - t138 + t158;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t184 = t84 * t84;
        let t193 = 2.0 * t51 + 2.0 * t124;
        let t195 = piecewise3(t5, t193, -t193);
        let t196 = piecewise3(t7, t195, 0.0);
        let tv2rho22 = 5.0 / 3.0 * t161 + t105 + 25.0 / 36.0 * t108 * t110 * t184 - 5.0 / 3.0 * t169 + 5.0 / 9.0 * t49 * t120 * t184 + 5.0 / 6.0 * t49 * t50 * t196 - t138 + t158;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t200 = t103 * t93;
        let t201 = t93 * t10;
        let t202 = t201 * t56;
        let t205 = t134 * t93;
        let t208 = 2.0 * t60 * t205 * t78;
        let t210 = 1.0 / t63 / t31;
        let t211 = t139 * t210;
        let t213 = t89 * t70;
        let t216 = -0.700380078125 * t211 - 0.32017375 * t213 - 0.03306 * t20;
        let t218 = t60 * t62 * t216;
        let tv2rhosigma0 = t200 - 5.0 / 6.0 * t115 * t202 - t208 + t218;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let t219 = t103 * t97;
        let t220 = t97 * t10;
        let t221 = t220 * t56;
        let t224 = t134 * t97;
        let t227 = 2.0 * t60 * t224 * t78;
        let t231 = -1.40076015625 * t211 - 0.6403475 * t213 - 0.06612 * t20;
        let t233 = t60 * t62 * t231;
        let tv2rhosigma1 = t219 - 5.0 / 6.0 * t115 * t221 - t227 + t233;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = tv2rhosigma0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t234 = t201 * t84;
        let tv2rhosigma3 = t200 - 5.0 / 6.0 * t115 * t234 - t208 + t218;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let t237 = t220 * t84;
        let tv2rhosigma4 = t219 - 5.0 / 6.0 * t115 * t237 - t227 + t233;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let tv2rhosigma5 = tv2rhosigma3;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t240 = t93 * t93;
        let t244 = 1.0 / t15;
        let t245 = t139 * t244;
        let t246 = t245 * t65;
        let t248 = 1.0 / t17;
        let t249 = t67 * t248;
        let t250 = t249 * t24;
        let t252 = 0.262642529296875 * t246 - 0.12006515625 * t250;
        let tv2sigma20 = -2.0 * t60 * t134 * t240 + t60 * t62 * t252;
        v2sigma2[ip * 6] += tv2sigma20;
        let t260 = 0.52528505859375 * t246 - 0.2401303125 * t250;
        let tv2sigma21 = -2.0 * t60 * t224 * t93 + t60 * t62 * t260;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = tv2sigma20;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let t263 = t97 * t97;
        let t269 = 1.0505701171875 * t246 - 0.480260625 * t250;
        let tv2sigma23 = -2.0 * t60 * t134 * t263 + t60 * t62 * t269;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = tv2sigma21;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = tv2sigma22;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
