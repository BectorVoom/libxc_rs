//! MGGA_X_2D_PRP10 fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 40 shared lines across all orders.
//! Delta: 73 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::bessel::{xc_bessel_I0, xc_bessel_I1};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_2d_prp10_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (40 lines) ---
        let t2 = rho0 * rho0;
        let t3 = 1.0 / t2;
        let t6 = tau0 * t3;
        let t8 = 1.0 / t2 / rho0;
        let t10 = sigma0 * t8 / 8.0;
        let t12 = 1.0 / M_PI;
        let t13 = (lapl0 * t3 / 4.0 - t6 + t10) * t12;
        let t14 = -0.9999999999e0 < t13;
        let t15 = piecewise3(t14, t13, -0.9999999999e0);
        let t16 = f64::exp(-1.0);
        let t18 = lambert_w(t15 * t16);
        let t19 = t18 + 1.0;
        let t20 = t19 / 2.0;
        let t21 = xc_bessel_I0(t20);
        let t23 = t6 - t10;
        let t24 = 0.1e-9 < t23;
        let t25 = piecewise3(t24, t23, 0.1e-9);
        let t26 = f64::sqrt(t25);
        let t29 = M_PI * t21 - 4.0 / 3.0 * t12 * t26;
        let t30 = f64::sqrt(rho0);
        let tvrho0 = -t29 * t30;
        vrho[ip * 2] += tvrho0;
        let t32 = rho1 * rho1;
        let t33 = 1.0 / t32;
        let t36 = tau1 * t33;
        let t38 = 1.0 / t32 / rho1;
        let t40 = sigma2 * t38 / 8.0;
        let t42 = (lapl1 * t33 / 4.0 - t36 + t40) * t12;
        let t43 = -0.9999999999e0 < t42;
        let t44 = piecewise3(t43, t42, -0.9999999999e0);
        let t46 = lambert_w(t44 * t16);
        let t47 = t46 + 1.0;
        let t48 = t47 / 2.0;
        let t49 = xc_bessel_I0(t48);
        let t51 = t36 - t40;
        let t52 = 0.1e-9 < t51;
        let t53 = piecewise3(t52, t51, 0.1e-9);
        let t54 = f64::sqrt(t53);
        let t57 = M_PI * t49 - 4.0 / 3.0 * t12 * t54;
        let t58 = f64::sqrt(rho1);
        let tvrho1 = -t57 * t58;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (this level) (73 lines) ---
        let t60 = xc_bessel_I1(t20);
        let t61 = M_PI * t60;
        let t65 = 2.0 * tau0 * t8;
        let t66 = t2 * t2;
        let t67 = 1.0 / t66;
        let t69 = 3.0 / 8.0 * sigma0 * t67;
        let t72 = piecewise3(t14, (-lapl0 * t8 / 2.0 + t65 - t69) * t12, 0.0);
        let t74 = 1.0 / t19;
        let t75 = t18 * t74;
        let t76 = 1.0 / t15;
        let t77 = t75 * t76;
        let t81 = t12 / t26;
        let t83 = piecewise3(t24, -t65 + t69, 0.0);
        let t86 = t61 * t72 * t77 / 2.0 - 2.0 / 3.0 * t81 * t83;
        let t88 = 1.0 / t30;
        let tv2rho20 = -t86 * t30 - t29 * t88 / 2.0;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = 0.0;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t91 = xc_bessel_I1(t48);
        let t92 = M_PI * t91;
        let t96 = 2.0 * tau1 * t38;
        let t97 = t32 * t32;
        let t98 = 1.0 / t97;
        let t100 = 3.0 / 8.0 * sigma2 * t98;
        let t103 = piecewise3(t43, (-lapl1 * t38 / 2.0 + t96 - t100) * t12, 0.0);
        let t105 = 1.0 / t47;
        let t106 = t46 * t105;
        let t107 = 1.0 / t44;
        let t108 = t106 * t107;
        let t112 = t12 / t54;
        let t114 = piecewise3(t52, -t96 + t100, 0.0);
        let t117 = t92 * t103 * t108 / 2.0 - 2.0 / 3.0 * t112 * t114;
        let t119 = 1.0 / t58;
        let tv2rho22 = -t117 * t58 - t57 * t119 / 2.0;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t122 = t3 * t12;
        let t124 = piecewise3(t14, t122 / 4.0, 0.0);
        let t125 = t61 * t124;
        let t127 = t75 * t76 * t30;
        let tv2rholapl0 = -t125 * t127 / 2.0;
        v2rholapl[ip * 4] += tv2rholapl0;
        let tv2rholapl1 = 0.0;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let tv2rholapl2 = 0.0;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t130 = t33 * t12;
        let t132 = piecewise3(t43, t130 / 4.0, 0.0);
        let t133 = t92 * t132;
        let t135 = t106 * t107 * t58;
        let tv2rholapl3 = -t133 * t135 / 2.0;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let t138 = t8 * t12;
        let t140 = piecewise3(t14, t138 / 8.0, 0.0);
        let t141 = t61 * t140;
        let t145 = piecewise3(t24, -t8 / 8.0, 0.0);
        let t148 = t141 * t77 / 2.0 - 2.0 / 3.0 * t81 * t145;
        let tv2rhosigma0 = -t148 * t30;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = 0.0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = 0.0;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t150 = t38 * t12;
        let t152 = piecewise3(t43, t150 / 8.0, 0.0);
        let t153 = t92 * t152;
        let t157 = piecewise3(t52, -t38 / 8.0, 0.0);
        let t160 = t153 * t108 / 2.0 - 2.0 / 3.0 * t112 * t157;
        let tv2rhosigma5 = -t160 * t58;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t162 = piecewise3(t14, -t122, 0.0);
        let t163 = t61 * t162;
        let t166 = piecewise3(t24, t3, 0.0);
        let t169 = t163 * t77 / 2.0 - 2.0 / 3.0 * t81 * t166;
        let tv2rhotau0 = -t169 * t30;
        v2rhotau[ip * 4] += tv2rhotau0;
        let tv2rhotau1 = 0.0;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let tv2rhotau2 = 0.0;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let t171 = piecewise3(t43, -t130, 0.0);
        let t172 = t92 * t171;
        let t175 = piecewise3(t52, t33, 0.0);
        let t178 = t172 * t108 / 2.0 - 2.0 / 3.0 * t112 * t175;
        let tv2rhotau3 = -t178 * t58;
        v2rhotau[ip * 4 + 3] += tv2rhotau3;
    }
}
