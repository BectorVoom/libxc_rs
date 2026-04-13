//! MGGA_X_PKZB vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pkzb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_pkzb_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = t34 * sigma0 * t39;
        let t44 = 1.0 / t37 / rho0;
        let t49 = t34 * tau0 * t44 / 4.0 - 9.0 / 20.0 - t41 / 288.0;
        let t50 = t49 * t49;
        let t52 = t49 * t29;
        let t53 = t33 * sigma0;
        let t54 = t53 * t39;
        let t57 = t29 * t29;
        let t59 = 1.0 / t31 / t30;
        let t60 = t57 * t59;
        let t61 = sigma0 * sigma0;
        let t62 = t35 * t35;
        let t63 = t62 * rho0;
        let t65 = 1.0 / t36 / t63;
        let t69 = 0.804e0 + 5.0 / 972.0 * t41 + 146.0 / 2025.0 * t50 - 73.0 / 9720.0 * t52 * t54 + 0.22909234000912809658e-3 * t60 * t61 * t65;
        let t72 = 0.1804e1 - 0.646416e0 / t69;
        let t76 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t26 * t27 * t72);
        let t77 = rho1 <= dens_threshold;
        let t78 = -t17;
        let t80 = piecewise5(t15, t12, t11, t16, t78 * t8);
        let t81 = 1.0 + t80;
        let t82 = t81 <= zeta_threshold;
        let t83 = pow_1_3(t81);
        let t85 = piecewise3(t82, t23, t83 * t81);
        let t87 = rho1 * rho1;
        let t88 = pow_1_3(rho1);
        let t89 = t88 * t88;
        let t91 = 1.0 / t89 / t87;
        let t93 = t34 * sigma2 * t91;
        let t96 = 1.0 / t89 / rho1;
        let t101 = t34 * tau1 * t96 / 4.0 - 9.0 / 20.0 - t93 / 288.0;
        let t102 = t101 * t101;
        let t104 = t101 * t29;
        let t105 = t33 * sigma2;
        let t106 = t105 * t91;
        let t109 = sigma2 * sigma2;
        let t110 = t87 * t87;
        let t111 = t110 * rho1;
        let t113 = 1.0 / t88 / t111;
        let t117 = 0.804e0 + 5.0 / 972.0 * t93 + 146.0 / 2025.0 * t102 - 73.0 / 9720.0 * t104 * t106 + 0.22909234000912809658e-3 * t60 * t109 * t113;
        let t120 = 0.1804e1 - 0.646416e0 / t117;
        let t124 = piecewise3(t77, 0.0, -3.0 / 8.0 * t6 * t85 * t27 * t120);
        let tzk0 = t76 + t124;
        zk[ip] += tzk0;
        let t125 = t7 * t7;
        let t126 = 1.0 / t125;
        let t127 = t17 * t126;
        let t129 = piecewise5(t11, 0.0, t15, 0.0, t8 - t127);
        let t132 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t129);
        let t137 = t27 * t27;
        let t138 = 1.0 / t137;
        let t142 = t6 * t26 * t138 * t72 / 8.0;
        let t143 = t3 * t26;
        let t144 = t69 * t69;
        let t145 = 1.0 / t144;
        let t146 = t27 * t145;
        let t147 = t35 * rho0;
        let t149 = 1.0 / t37 / t147;
        let t151 = t34 * sigma0 * t149;
        let t157 = -5.0 / 12.0 * t34 * tau0 * t39 + t151 / 108.0;
        let t160 = t157 * t29;
        let t163 = t53 * t149;
        let t166 = t62 * t35;
        let t168 = 1.0 / t36 / t166;
        let t172 = -10.0 / 729.0 * t151 + 292.0 / 2025.0 * t49 * t157 - 73.0 / 9720.0 * t160 * t54 + 73.0 / 3645.0 * t52 * t163 - 0.12218258133820165151e-2 * t60 * t61 * t168;
        let t173 = t146 * t172;
        let t177 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t132 * t27 * t72 - t142 - 0.16551095363746320496e0 * t143 * t173);
        let t178 = t78 * t126;
        let t180 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t178);
        let t183 = piecewise3(t82, 0.0, 4.0 / 3.0 * t83 * t180);
        let t191 = t6 * t85 * t138 * t120 / 8.0;
        let t193 = piecewise3(t77, 0.0, -3.0 / 8.0 * t6 * t183 * t27 * t120 - t191);
        let tvrho0 = t76 + t124 + t7 * (t177 + t193);
        vrho[ip * 2] += tvrho0;
        let t197 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t127);
        let t200 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t197);
        let t206 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t200 * t27 * t72 - t142);
        let t208 = piecewise5(t15, 0.0, t11, 0.0, t8 - t178);
        let t211 = piecewise3(t82, 0.0, 4.0 / 3.0 * t83 * t208);
        let t216 = t3 * t85;
        let t217 = t117 * t117;
        let t218 = 1.0 / t217;
        let t219 = t27 * t218;
        let t220 = t87 * rho1;
        let t222 = 1.0 / t89 / t220;
        let t224 = t34 * sigma2 * t222;
        let t230 = -5.0 / 12.0 * t34 * tau1 * t91 + t224 / 108.0;
        let t233 = t230 * t29;
        let t236 = t105 * t222;
        let t239 = t110 * t87;
        let t241 = 1.0 / t88 / t239;
        let t245 = -10.0 / 729.0 * t224 + 292.0 / 2025.0 * t101 * t230 - 73.0 / 9720.0 * t233 * t106 + 73.0 / 3645.0 * t104 * t236 - 0.12218258133820165151e-2 * t60 * t109 * t241;
        let t246 = t219 * t245;
        let t250 = piecewise3(t77, 0.0, -3.0 / 8.0 * t6 * t211 * t27 * t120 - t191 - 0.16551095363746320496e0 * t216 * t246);
        let tvrho1 = t76 + t124 + t7 * (t206 + t250);
        vrho[ip * 2 + 1] += tvrho1;
        let t255 = t33 * t39;
        let t256 = t52 * t255;
        let t259 = t60 * t65 * sigma0;
        let t261 = 5.0 / 972.0 * t34 * t39 - 146.0 / 18225.0 * t256 + 0.48426206913576876746e-3 * t259;
        let t262 = t146 * t261;
        let t265 = piecewise3(t2, 0.0, -0.16551095363746320496e0 * t143 * t262);
        let tvsigma0 = t7 * t265;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t268 = t33 * t91;
        let t269 = t104 * t268;
        let t272 = t60 * t113 * sigma2;
        let t274 = 5.0 / 972.0 * t34 * t91 - 146.0 / 18225.0 * t269 + 0.48426206913576876746e-3 * t272;
        let t275 = t219 * t274;
        let t278 = piecewise3(t77, 0.0, -0.16551095363746320496e0 * t216 * t275);
        let tvsigma2 = t7 * t278;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t279 = t33 * t44;
        let t283 = 1.0 / t36 / t62;
        let t287 = 73.0 / 2025.0 * t52 * t279 - 73.0 / 38880.0 * t60 * t283 * sigma0;
        let t288 = t146 * t287;
        let t291 = piecewise3(t2, 0.0, -0.16551095363746320496e0 * t143 * t288);
        let tvtau0 = t7 * t291;
        vtau[ip * 2] += tvtau0;
        let t292 = t33 * t96;
        let t296 = 1.0 / t88 / t110;
        let t300 = 73.0 / 2025.0 * t104 * t292 - 73.0 / 38880.0 * t60 * t296 * sigma2;
        let t301 = t219 * t300;
        let t304 = piecewise3(t77, 0.0, -0.16551095363746320496e0 * t216 * t301);
        let tvtau1 = t7 * t304;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
