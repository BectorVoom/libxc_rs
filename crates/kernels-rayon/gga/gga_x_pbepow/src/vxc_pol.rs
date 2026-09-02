//! GGA_X_PBEPOW vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbepow_vxc_pol(
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
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t39 = sigma0 * t38;
        let t40 = t33 * t39;
        let t42 = 0.9146457198521546 * t40 + 0.804;
        let t43 = 1.0 / t42;
        let t45 = t33 * t39 * t43;
        let t46 = rmath::pow(t45, 100.0);
        let t48 = 0.0001334414156799501 * t46 - 1.0;
        let t52 = 1.0 - 0.009146457198521547 * t33 * t39 * t48;
        let t56 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t16;
        let t60 = piecewise5(t14, t11, t10, t15, t58 * t7);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t65 = piecewise3(t62, t22, t63 * t61);
        let t66 = t65 * t26;
        let t67 = rho1 * rho1;
        let t68 = pow_1_3(rho1);
        let t69 = t68 * t68;
        let t71 = 1.0 / t69 / t67;
        let t72 = sigma2 * t71;
        let t73 = t33 * t72;
        let t75 = 0.9146457198521546 * t73 + 0.804;
        let t76 = 1.0 / t75;
        let t78 = t33 * t72 * t76;
        let t79 = rmath::pow(t78, 100.0);
        let t81 = 0.0001334414156799501 * t79 - 1.0;
        let t85 = 1.0 - 0.009146457198521547 * t33 * t72 * t81;
        let t89 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t66 * t85);
        let tzk0 = t56 + t89;
        zk[ip] += tzk0;
        let t90 = t6 * t6;
        let t91 = 1.0 / t90;
        let t92 = t16 * t91;
        let t94 = piecewise5(t10, 0.0, t14, 0.0, t7 - t92);
        let t97 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t94);
        let t98 = t97 * t26;
        let t102 = t26 * t26;
        let t103 = 1.0 / t102;
        let t104 = t25 * t103;
        let t107 = t5 * t104 * t52 / 8.0;
        let t108 = t34 * rho0;
        let t110 = 1.0 / t36 / t108;
        let t111 = sigma0 * t110;
        let t115 = t33 * sigma0;
        let t116 = rmath::pow(t45, 99.0);
        let t117 = t38 * t116;
        let t121 = t28 * t28;
        let t123 = 1.0 / t30 / t29;
        let t124 = t121 * t123;
        let t125 = sigma0 * sigma0;
        let t126 = t34 * t34;
        let t127 = t126 * t34;
        let t129 = 1.0 / t35 / t127;
        let t131 = t42 * t42;
        let t132 = 1.0 / t131;
        let t136 = -8.0 / 3.0 * t33 * t111 * t43 + 2.4390552529390788 * t124 * t125 * t129 * t132;
        let t137 = t117 * t136;
        let t140 = 0.024390552529390788 * t33 * t111 * t48 - 0.00012205161970267855 * t115 * t137;
        let t145 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t98 * t52 - t107 - 3.0 / 8.0 * t5 * t27 * t140);
        let t146 = t58 * t91;
        let t148 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t146);
        let t151 = piecewise3(t62, 0.0, 4.0 / 3.0 * t63 * t148);
        let t152 = t151 * t26;
        let t156 = t65 * t103;
        let t159 = t5 * t156 * t85 / 8.0;
        let t161 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t152 * t85 - t159);
        let tvrho0 = t56 + t89 + t6 * (t145 + t161);
        vrho[ip * 2] += tvrho0;
        let t165 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t92);
        let t168 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t165);
        let t169 = t168 * t26;
        let t174 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t169 * t52 - t107);
        let t176 = piecewise5(t14, 0.0, t10, 0.0, t7 - t146);
        let t179 = piecewise3(t62, 0.0, 4.0 / 3.0 * t63 * t176);
        let t180 = t179 * t26;
        let t184 = t67 * rho1;
        let t186 = 1.0 / t69 / t184;
        let t187 = sigma2 * t186;
        let t191 = t33 * sigma2;
        let t192 = rmath::pow(t78, 99.0);
        let t193 = t71 * t192;
        let t197 = sigma2 * sigma2;
        let t198 = t67 * t67;
        let t199 = t198 * t67;
        let t201 = 1.0 / t68 / t199;
        let t203 = t75 * t75;
        let t204 = 1.0 / t203;
        let t208 = -8.0 / 3.0 * t33 * t187 * t76 + 2.4390552529390788 * t124 * t197 * t201 * t204;
        let t209 = t193 * t208;
        let t212 = 0.024390552529390788 * t33 * t187 * t81 - 0.00012205161970267855 * t191 * t209;
        let t217 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t180 * t85 - t159 - 3.0 / 8.0 * t5 * t66 * t212);
        let tvrho1 = t56 + t89 + t6 * (t174 + t217);
        vrho[ip * 2 + 1] += tvrho1;
        let t225 = t126 * rho0;
        let t227 = 1.0 / t35 / t225;
        let t232 = t33 * t38 * t43 - 0.9146457198521546 * t124 * sigma0 * t227 * t132;
        let t233 = t117 * t232;
        let t236 = -0.009146457198521547 * t33 * t38 * t48 - 0.00012205161970267855 * t115 * t233;
        let t240 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t236);
        let tvsigma0 = t6 * t240;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t246 = t198 * rho1;
        let t248 = 1.0 / t68 / t246;
        let t253 = t33 * t71 * t76 - 0.9146457198521546 * t124 * sigma2 * t248 * t204;
        let t254 = t193 * t253;
        let t257 = -0.009146457198521547 * t33 * t71 * t81 - 0.00012205161970267855 * t191 * t254;
        let t261 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t66 * t257);
        let tvsigma2 = t6 * t261;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
