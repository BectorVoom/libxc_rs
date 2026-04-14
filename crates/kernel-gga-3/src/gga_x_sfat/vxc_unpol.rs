//! GGA_X_SFAT vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 85 shared lines across all orders.
//! Delta: 99 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sfat_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (85 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t17 / t4 * t3;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t25 * t24;
        let t27 = t24 * t20;
        let t28 = t25 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = t30 * sigma[ip];
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t35 = 1.0 / t33 / t32;
        let t36 = f64::sqrt(sigma[ip]);
        let t37 = t29 * t36;
        let t39 = 1.0 / t19 / rho[ip];
        let t41 = f64::ln(t39 * t37 + f64::sqrt(pow_2(t39 * t37) + 1.0));
        let t42 = t41 * t39;
        let t45 = 1.0 + 0.252e-1 * t42 * t37;
        let t46 = 1.0 / t45;
        let t51 = 1.0 + 0.93333333333333333332e-3 * t46 * t35 * t31 * t28;
        let t54 = 1.0 / t51 * t26 * t20 * M_PI;
        let t55 = f64::sqrt(t54);
        let t57 = 1.0 / t55 * param_hyb_omega_0;
        let t58 = rho[ip] * t11;
        let t59 = pow_1_3(t58);
        let t60 = 1.0 / t59;
        let t61 = t60 * t29;
        let t63 = t61 * t57 / 2.0;
        let t64 = 0.192e1 <= t63;
        let t65 = 0.192e1 < t63;
        let t66 = piecewise3(t65, t63, 0.192e1);
        let t67 = t66 * t66;
        let t68 = t67 * t67;
        let t69 = t68 * t68;
        let t70 = t69 * t69;
        let t71 = t70 * t70;
        let t73 = 1.0 / t71 / t67;
        let t76 = 1.0 / t71 / t68;
        let t78 = 1.0 / t68;
        let t80 = t68 * t67;
        let t81 = 1.0 / t80;
        let t83 = 1.0 / t69;
        let t85 = t69 * t67;
        let t86 = 1.0 / t85;
        let t88 = t69 * t68;
        let t89 = 1.0 / t88;
        let t91 = t69 * t80;
        let t92 = 1.0 / t91;
        let t94 = 1.0 / t70;
        let t97 = 1.0 / t70 / t67;
        let t100 = 1.0 / t70 / t68;
        let t103 = 1.0 / t70 / t80;
        let t106 = 1.0 / t70 / t69;
        let t109 = 1.0 / t70 / t85;
        let t112 = 1.0 / t70 / t88;
        let t115 = 1.0 / t70 / t91;
        let t117 = 1.0 / t71;
        let t121 = t73 / 5985.0 - t76 / 7030.0 - t78 / 30.0 + t81 / 70.0 - t83 / 135.0 + t86 / 231.0 - t89 / 364.0 + t92 / 540.0 - t94 / 765.0 + t97 / 1045.0 - t100 / 1386.0 + t103 / 1794.0 - t106 / 2275.0 + t109 / 2835.0 - t112 / 3480.0 + t115 / 4216.0 - t117 / 5049.0 + 1.0 / t67 / 9.0;
        let t122 = piecewise3(t65, 0.192e1, t63);
        let t123 = f64::atan2(1.0, t122);
        let t124 = t122 * t122;
        let t125 = t124 + 3.0;
        let t126 = 1.0 / t124;
        let t127 = 1.0 + t126;
        let t128 = f64::ln(t127);
        let t130 = -t125 * t128 + 1.0;
        let t133 = t123 + t130 * t122 / 4.0;
        let t137 = piecewise3(t64, t121, 1.0 - 8.0 / 3.0 * t133 * t122);
        let t138 = t137 * t19;
        let t142 = piecewise3(t2, 0.0, -3.0 / 8.0 * t51 * t138 * t18);
        let tzk0 = 2.0 * t142;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (99 lines) ---
        let t143 = 1.0 / t33;
        let t144 = t137 * t143;
        let t148 = t67 * t66;
        let t150 = 1.0 / t71 / t148;
        let t153 = 1.0 / t55 / t54 * param_hyb_omega_0;
        let t155 = M_PI * t61 * t153;
        let t156 = t51 * t51;
        let t157 = 1.0 / t156;
        let t158 = t157 * t25;
        let t159 = t32 * rho[ip];
        let t161 = 1.0 / t33 / t159;
        let t166 = sigma[ip] * t25;
        let t167 = t166 * t27;
        let t168 = t35 * t30;
        let t169 = t45 * t45;
        let t170 = 1.0 / t169;
        let t173 = t41 / t19 / t32;
        let t177 = t31 * t35 + 1.0;
        let t178 = f64::sqrt(t177);
        let t179 = 1.0 / t178;
        let t180 = t179 * t161;
        let t183 = -0.336e-1 * t173 * t37 - 0.336e-1 * t180 * t31;
        let t184 = t183 * t170;
        let t185 = t184 * t168;
        let t188 = -0.24888888888888888889e-2 * t46 * t161 * t31 * t28 - 0.93333333333333333332e-3 * t185 * t167;
        let t194 = 1.0 / t59 / t58;
        let t195 = t194 * t29;
        let t199 = t188 * t158 * t27 * t155 / 4.0 - t11 * t195 * t57 / 6.0;
        let t200 = piecewise3(t65, t199, 0.0);
        let t203 = t68 * t66;
        let t205 = 1.0 / t71 / t203;
        let t208 = 1.0 / t203;
        let t211 = t68 * t148;
        let t212 = 1.0 / t211;
        let t215 = t69 * t66;
        let t216 = 1.0 / t215;
        let t219 = t69 * t148;
        let t220 = 1.0 / t219;
        let t223 = t69 * t203;
        let t224 = 1.0 / t223;
        let t227 = t69 * t211;
        let t228 = 1.0 / t227;
        let t232 = 1.0 / t70 / t66;
        let t236 = 1.0 / t70 / t148;
        let t240 = 1.0 / t70 / t203;
        let t244 = 1.0 / t70 / t211;
        let t248 = 1.0 / t70 / t215;
        let t252 = 1.0 / t70 / t219;
        let t256 = 1.0 / t70 / t223;
        let t260 = 1.0 / t70 / t227;
        let t264 = 1.0 / t71 / t66;
        let t267 = 1.0 / t148;
        let t270 = -34.0 / 5985.0 * t200 * t150 + 18.0 / 3515.0 * t200 * t205 + 2.0 / 15.0 * t200 * t208 - 3.0 / 35.0 * t200 * t212 + 8.0 / 135.0 * t200 * t216 - 10.0 / 231.0 * t200 * t220 + 3.0 / 91.0 * t200 * t224 - 7.0 / 270.0 * t200 * t228 + 16.0 / 765.0 * t200 * t232 - 18.0 / 1045.0 * t200 * t236 + 10.0 / 693.0 * t200 * t240 - 11.0 / 897.0 * t200 * t244 + 24.0 / 2275.0 * t200 * t248 - 26.0 / 2835.0 * t200 * t252 + 7.0 / 870.0 * t200 * t256 - 15.0 / 2108.0 * t200 * t260 + 32.0 / 5049.0 * t200 * t264 - 2.0 / 9.0 * t200 * t267;
        let t271 = piecewise3(t65, 0.0, t199);
        let t274 = 1.0 / t127;
        let t280 = t124 * t122;
        let t281 = 1.0 / t280;
        let t282 = t281 * t125;
        let t283 = t274 * t271;
        let t286 = -2.0 * t122 * t128 * t271 + 2.0 * t282 * t283;
        let t289 = -t274 * t126 * t271 + t130 * t271 / 4.0 + t286 * t122 / 4.0;
        let t293 = piecewise3(t64, t270, -8.0 / 3.0 * t289 * t122 - 8.0 / 3.0 * t133 * t271);
        let t294 = t293 * t19;
        let t302 = piecewise3(t2, 0.0, -t51 * t144 * t18 / 8.0 - 3.0 / 8.0 * t51 * t294 * t18 - 3.0 / 8.0 * t188 * t138 * t18);
        let tvrho0 = 2.0 * rho[ip] * t302 + 2.0 * t142;
        vrho[ip] += tvrho0;
        let t309 = t29 / t36;
        let t314 = 0.126e-1 * t42 * t309 + 0.126e-1 * t179 * t168;
        let t315 = t314 * t170;
        let t316 = t315 * t168;
        let t319 = 0.93333333333333333332e-3 * t46 * t168 * t28 - 0.93333333333333333332e-3 * t316 * t167;
        let t323 = t319 * t158 * t27 * t155 / 4.0;
        let t324 = piecewise3(t65, t323, 0.0);
        let t325 = t324 * t150;
        let t327 = t324 * t205;
        let t329 = t324 * t208;
        let t331 = t324 * t212;
        let t333 = t324 * t216;
        let t335 = t324 * t220;
        let t337 = t324 * t224;
        let t339 = t324 * t228;
        let t341 = t324 * t232;
        let t343 = t324 * t236;
        let t345 = t324 * t240;
        let t347 = t324 * t244;
        let t349 = t324 * t248;
        let t351 = t324 * t252;
        let t353 = t324 * t256;
        let t355 = t324 * t260;
        let t357 = t324 * t264;
        let t361 = -34.0 / 5985.0 * t325 + 18.0 / 3515.0 * t327 + 2.0 / 15.0 * t329 - 3.0 / 35.0 * t331 + 8.0 / 135.0 * t333 - 10.0 / 231.0 * t335 + 3.0 / 91.0 * t337 - 7.0 / 270.0 * t339 + 16.0 / 765.0 * t341 - 18.0 / 1045.0 * t343 + 10.0 / 693.0 * t345 - 11.0 / 897.0 * t347 + 24.0 / 2275.0 * t349 - 26.0 / 2835.0 * t351 + 7.0 / 870.0 * t353 - 15.0 / 2108.0 * t355 + 32.0 / 5049.0 * t357 - 2.0 / 9.0 * t324 * t267;
        let t362 = piecewise3(t65, 0.0, t323);
        let t364 = t126 * t362;
        let t370 = t274 * t362;
        let t373 = -2.0 * t122 * t128 * t362 + 2.0 * t282 * t370;
        let t376 = -t274 * t364 + t130 * t362 / 4.0 + t373 * t122 / 4.0;
        let t380 = piecewise3(t64, t361, -8.0 / 3.0 * t376 * t122 - 8.0 / 3.0 * t133 * t362);
        let t381 = t380 * t19;
        let t388 = piecewise3(t2, 0.0, -3.0 / 8.0 * t319 * t138 * t18 - 3.0 / 8.0 * t51 * t381 * t18);
        let tvsigma0 = 2.0 * t388 * rho[ip];
        vsigma[ip] += tvsigma0;
    }
}
