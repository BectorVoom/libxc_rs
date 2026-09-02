//! GGA_X_AM05 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_am05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::lambert_w::{lambert_w};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_am05_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = param_alpha * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t37 = 1.0 + t35 / 24.0;
        let t38 = 1.0 / t37;
        let t39 = t33 * t38;
        let t43 = t25 * sigma[ip];
        let t44 = t21 * t43;
        let t45 = t28 * t33;
        let t46 = param_c * t20;
        let t47 = t46 * t25;
        let t50 = 1.0 + t47 * t34 / 24.0;
        let t51 = t38 * t50;
        let t52 = t20 * t20;
        let t53 = param_c * t52;
        let t54 = 1.0 / t23;
        let t55 = rmath::sqrt(sigma[ip]);
        let t56 = t54 * t55;
        let t58 = t53 * t56 * t28;
        let t60 = 1.0 / t18 / rho[ip];
        let t61 = 1.0 / M_PI;
        let t62 = t60 * t61;
        let t63 = t3 * t3;
        let t64 = rmath::sqrt(12.0);
        let t68 = t52 * t54 * t55 * t27 * t60;
        let t69 = rmath::sqrt(t68);
        let t72 = rmath::sqrt(6.0);
        let t75 = lambert_w(t64 * t69 * t68 * t72 / 1728.0);
        let t76 = pow_1_3(t75);
        let t77 = t76 * t76;
        let t83 = 28.23705740248932 + 3.0 / 4.0 * t3 * t28 * t76 * t75;
        let t84 = pow_1_4(t83);
        let t85 = t63 * t77 * t84;
        let t86 = t62 * t85;
        let t89 = 1.0 + t58 * t86 / 8.0;
        let t90 = 1.0 / t89;
        let t91 = t51 * t90;
        let t95 = 1.0 - t26 * t29 * t39 / 24.0 + t44 * t45 * t91 / 24.0;
        let t99 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t95);
        let tzk0 = 2.0 * t99;
        zk[ip] += tzk0;
        let t101 = t17 / t31;
        let t105 = t30 * rho[ip];
        let t107 = 1.0 / t31 / t105;
        let t108 = t107 * t38;
        let t112 = param_alpha * param_alpha;
        let t113 = t112 * t52;
        let t115 = 1.0 / t23 / t22;
        let t116 = t113 * t115;
        let t117 = sigma[ip] * sigma[ip];
        let t118 = t117 * t27;
        let t119 = t30 * t30;
        let t120 = t119 * t30;
        let t122 = 1.0 / t18 / t120;
        let t123 = t37 * t37;
        let t124 = 1.0 / t123;
        let t125 = t122 * t124;
        let t129 = t28 * t107;
        let t133 = t115 * t117;
        let t134 = t113 * t133;
        let t135 = t27 * t122;
        let t136 = t124 * t50;
        let t137 = t136 * t90;
        let t141 = param_alpha * t52;
        let t142 = t141 * t133;
        let t143 = t38 * param_c;
        let t144 = t143 * t90;
        let t149 = t21 * t43 * t28;
        let t150 = t89 * t89;
        let t151 = 1.0 / t150;
        let t152 = t50 * t151;
        let t154 = 1.0 / t18 / t30;
        let t155 = t154 * t61;
        let t156 = t155 * t85;
        let t159 = t155 * t63;
        let t160 = t77 * t84;
        let t161 = 1.0 + t75;
        let t162 = 1.0 / t161;
        let t163 = t160 * t162;
        let t164 = t159 * t163;
        let t168 = t53 * t56 * t27;
        let t169 = t75 * t75;
        let t170 = t84 * t84;
        let t171 = t170 * t84;
        let t172 = 1.0 / t171;
        let t173 = t169 * t172;
        let t174 = t173 * t162;
        let t175 = t155 * t174;
        let t178 = -t58 * t156 / 6.0 - t58 * t164 / 6.0 - 3.0 / 8.0 * t168 * t175;
        let t179 = t152 * t178;
        let t180 = t39 * t179;
        let t183 = t26 * t29 * t108 / 9.0 - t116 * t118 * t125 / 108.0 - t44 * t129 * t91 / 9.0 + t134 * t135 * t137 / 108.0 - t142 * t135 * t144 / 108.0 - t149 * t180 / 24.0;
        let t188 = piecewise3(t2, 0.0, -t6 * t101 * t95 / 8.0 - 3.0 / 8.0 * t6 * t19 * t183);
        let tvrho0 = 2.0 * rho[ip] * t188 + 2.0 * t99;
        vrho[ip] += tvrho0;
        let t195 = t119 * rho[ip];
        let t197 = 1.0 / t18 / t195;
        let t198 = t197 * t124;
        let t202 = t25 * t28;
        let t203 = t21 * t202;
        let t204 = t50 * t90;
        let t208 = t115 * sigma[ip];
        let t210 = t27 * t197;
        let t219 = t54 / t55;
        let t221 = t53 * t219 * t28;
        let t224 = t62 * t63;
        let t225 = t224 * t163;
        let t229 = t53 * t219 * t27;
        let t230 = t62 * t174;
        let t233 = t221 * t86 / 16.0 + t221 * t225 / 16.0 + 9.0 / 64.0 * t229 * t230;
        let t234 = t152 * t233;
        let t235 = t39 * t234;
        let t238 = -t26 * t45 * t38 / 24.0 + t116 * sigma[ip] * t27 * t198 / 288.0 + t203 * t39 * t204 / 24.0 - t113 * t208 * t210 * t137 / 288.0 + t141 * t208 * t210 * t144 / 288.0 - t149 * t235 / 24.0;
        let t242 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t238);
        let tvsigma0 = 2.0 * rho[ip] * t242;
        vsigma[ip] += tvsigma0;
    }
}
