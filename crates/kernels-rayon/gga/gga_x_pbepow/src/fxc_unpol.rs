//! GGA_X_PBEPOW fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbepow_fxc_unpol(
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
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = sigma[ip] * t28;
        let t38 = 0.9146457198521546 * t25 * t34 * t32 + 0.804;
        let t39 = 1.0 / t38;
        let t40 = t33 * t39;
        let t41 = t26 * t40;
        let t42 = rmath::pow(t41, 100.0);
        let t44 = 0.0001334414156799501 * t42 - 1.0;
        let t45 = t33 * t44;
        let t48 = 1.0 - 0.009146457198521547 * t26 * t45;
        let t52 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t48);
        let tzk0 = 2.0 * t52;
        zk[ip] += tzk0;
        let t54 = t17 / t30;
        let t58 = t29 * rho[ip];
        let t60 = 1.0 / t30 / t58;
        let t61 = t28 * t60;
        let t62 = t61 * t44;
        let t65 = rmath::pow(t41, 99.0);
        let t66 = t61 * t39;
        let t69 = t20 * t20;
        let t72 = t69 / t22 / t21;
        let t73 = sigma[ip] * sigma[ip];
        let t74 = t72 * t73;
        let t75 = t29 * t29;
        let t76 = t75 * t29;
        let t78 = 1.0 / t18 / t76;
        let t80 = t38 * t38;
        let t81 = 1.0 / t80;
        let t82 = t27 * t78 * t81;
        let t85 = -8.0 / 3.0 * t26 * t66 + 4.8781105058781575 * t74 * t82;
        let t86 = t65 * t85;
        let t90 = 0.024390552529390788 * t26 * t62 - 0.00012205161970267855 * t26 * t33 * t86;
        let t95 = piecewise3(t2, 0.0, -t6 * t54 * t48 / 8.0 - 3.0 / 8.0 * t6 * t19 * t90);
        let tvrho0 = 2.0 * rho[ip] * t95 + 2.0 * t52;
        vrho[ip] += tvrho0;
        let t102 = t75 * rho[ip];
        let t106 = t27 / t18 / t102 * t81;
        let t109 = t25 * t40 - 1.8292914397043092 * t72 * sigma[ip] * t106;
        let t110 = t65 * t109;
        let t114 = -0.009146457198521547 * t25 * t45 - 0.00012205161970267855 * t26 * t33 * t110;
        let t118 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t114);
        let tvsigma0 = 2.0 * rho[ip] * t118;
        vsigma[ip] += tvsigma0;
        let t123 = t17 / t30 / rho[ip];
        let t131 = 1.0 / t30 / t75;
        let t132 = t28 * t131;
        let t133 = t132 * t44;
        let t139 = rmath::pow(t41, 98.0);
        let t140 = t85 * t85;
        let t141 = t139 * t140;
        let t145 = t132 * t39;
        let t148 = t75 * t58;
        let t150 = 1.0 / t18 / t148;
        let t152 = t27 * t150 * t81;
        let t155 = t73 * sigma[ip];
        let t156 = t75 * t75;
        let t157 = t156 * t29;
        let t158 = 1.0 / t157;
        let t161 = 1.0 / t80 / t38;
        let t164 = 88.0 / 9.0 * t26 * t145 - 43.90299455290342 * t74 * t152 + 2.931467096752081 * t155 * t158 * t161;
        let t165 = t65 * t164;
        let t169 = -0.08943202594109956 * t26 * t133 + 0.0006509419717476189 * t26 * t61 * t86 - 0.012083110350565177 * t26 * t33 * t141 - 0.00012205161970267855 * t26 * t33 * t165;
        let t174 = piecewise3(t2, 0.0, t6 * t123 * t48 / 12.0 - t6 * t54 * t90 / 4.0 - 3.0 / 8.0 * t6 * t19 * t169);
        let tv2rho20 = 2.0 * rho[ip] * t174 + 4.0 * t95;
        v2rho2[ip] += tv2rho20;
        let t182 = t25 * t28;
        let t183 = t32 * t65;
        let t190 = t25 * t34;
        let t191 = t32 * t139;
        let t192 = t109 * t85;
        let t193 = t191 * t192;
        let t198 = t72 * t27;
        let t203 = t156 * rho[ip];
        let t204 = 1.0 / t203;
        let t208 = -8.0 / 3.0 * t25 * t66 + 14.634331517634473 * t198 * t78 * t81 * sigma[ip] - 1.0993001612820303 * t73 * t204 * t161;
        let t209 = t65 * t208;
        let t213 = 0.024390552529390788 * t25 * t62 - 0.00012205161970267855 * t182 * t183 * t85 + 0.00032547098587380947 * t26 * t61 * t110 - 0.012083110350565177 * t190 * t193 - 0.00012205161970267855 * t26 * t33 * t209;
        let t218 = piecewise3(t2, 0.0, -t6 * t54 * t114 / 8.0 - 3.0 / 8.0 * t6 * t19 * t213);
        let tv2rhosigma0 = 2.0 * rho[ip] * t218 + 2.0 * t118;
        v2rhosigma[ip] += tv2rhosigma0;
        let t224 = t109 * t109;
        let t225 = t139 * t224;
        let t231 = 1.0 / t156;
        let t235 = -3.6585828794086184 * t72 * t106 + 0.4122375604807614 * sigma[ip] * t231 * t161;
        let t236 = t65 * t235;
        let t240 = -0.0002441032394053571 * t182 * t183 * t109 - 0.012083110350565177 * t26 * t33 * t225 - 0.00012205161970267855 * t26 * t33 * t236;
        let t244 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t240);
        let tv2sigma20 = 2.0 * rho[ip] * t244;
        v2sigma2[ip] += tv2sigma20;
    }
}
