//! MGGA_K_PGSLB fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pgslb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_pgslb_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_pgslb_mu: f64,
    param_pgslb_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t37 = t33 * t36;
        let t41 = param_pgslb_mu * t25 * t29;
        let t44 = rmath::exp(-t41 * t37 / 24.0);
        let t45 = t25 * t25;
        let t46 = param_pgslb_beta * t45;
        let t48 = 1.0 / t27 / t26;
        let t49 = t46 * t48;
        let t50 = lapl[ip] * lapl[ip];
        let t51 = t50 * t31;
        let t52 = t34 * rho[ip];
        let t54 = 1.0 / t22 / t52;
        let t58 = 5.0 / 72.0 * t30 * t37 + t44 + t49 * t51 * t54 / 288.0;
        let t62 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t58);
        let tzk0 = 2.0 * t62;
        zk[ip] += tzk0;
        let t64 = t21 / t22;
        let t69 = 1.0 / t23 / t52;
        let t77 = t34 * t34;
        let t83 = -5.0 / 27.0 * t30 * t33 * t69 + t41 * t33 * t69 * t44 / 9.0 - 5.0 / 432.0 * t49 * t51 / t22 / t77;
        let t88 = piecewise3(t3, 0.0, t8 * t64 * t58 / 10.0 + 3.0 / 20.0 * t8 * t24 * t83);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t62;
        vrho[ip] += tvrho0;
        let t91 = t32 * t36;
        let t97 = 5.0 / 72.0 * t30 * t91 - t41 * t91 * t44 / 24.0;
        let t101 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t97);
        let tvsigma0 = 2.0 * rho[ip] * t101;
        vsigma[ip] += tvsigma0;
        let t104 = t8 * t21 * t36;
        let t107 = t46 * t48 * lapl[ip] * t31;
        let t110 = piecewise3(t3, 0.0, t104 * t107 / 960.0);
        let tvlapl0 = 2.0 * rho[ip] * t110;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t115 = t21 / t22 / rho[ip];
        let t123 = 1.0 / t23 / t77;
        let t131 = param_pgslb_mu * param_pgslb_mu;
        let t132 = t131 * t45;
        let t133 = t132 * t48;
        let t134 = sigma[ip] * sigma[ip];
        let t135 = t134 * t31;
        let t138 = 1.0 / t22 / t77 / t52;
        let t143 = t77 * rho[ip];
        let t149 = 55.0 / 81.0 * t30 * t33 * t123 - 11.0 / 27.0 * t41 * t33 * t123 * t44 + 2.0 / 81.0 * t133 * t135 * t138 * t44 + 65.0 / 1296.0 * t49 * t51 / t22 / t143;
        let t154 = piecewise3(t3, 0.0, -t8 * t115 * t58 / 30.0 + t8 * t64 * t83 / 5.0 + 3.0 / 20.0 * t8 * t24 * t149);
        let tv2rho20 = 2.0 * rho[ip] * t154 + 4.0 * t88;
        v2rho2[ip] += tv2rho20;
        let t160 = t32 * t69;
        let t166 = t77 * t34;
        let t168 = 1.0 / t22 / t166;
        let t170 = sigma[ip] * t44;
        let t174 = -5.0 / 27.0 * t30 * t160 + t41 * t160 * t44 / 9.0 - t133 * t31 * t168 * t170 / 108.0;
        let t179 = piecewise3(t3, 0.0, t8 * t64 * t97 / 10.0 + 3.0 / 20.0 * t8 * t24 * t174);
        let tv2rhosigma0 = 2.0 * rho[ip] * t179 + 2.0 * t101;
        v2rhosigma[ip] += tv2rhosigma0;
        let t183 = t8 * t21 * t69;
        let t186 = piecewise3(t3, 0.0, -t183 * t107 / 360.0);
        let tv2rholapl0 = 2.0 * rho[ip] * t186 + 2.0 * t110;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t190 = t8 * t21 * t123;
        let t191 = t48 * t31;
        let t193 = t132 * t191 * t44;
        let t196 = piecewise3(t3, 0.0, t190 * t193 / 1920.0);
        let tv2sigma20 = 2.0 * rho[ip] * t196;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t198 = t46 * t191;
        let t201 = piecewise3(t3, 0.0, t104 * t198 / 960.0);
        let tv2lapl20 = 2.0 * rho[ip] * t201;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
