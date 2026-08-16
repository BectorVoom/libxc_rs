//! MGGA_X_2D_PRP10 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_vxc/mgga_x_2d_prp10.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::bessel::{xc_bessel_I0, xc_bessel_I1};
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::lambert_w::{lambert_w};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_2d_prp10_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..vrho.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = 1.0 / t2;
        let t7 = 2.0 * tau[ip] * t3;
        let t9 = 1.0 / t2 / rho[ip];
        let t11 = sigma[ip] * t9 / 4.0;
        let t13 = 1.0 / M_PI;
        let t14 = (lapl[ip] * t3 / 2.0 - t7 + t11) * t13;
        let t15 = -0.9999999999e0 < t14;
        let t16 = piecewise3(t15, t14, -0.9999999999e0);
        let t17 = f64::exp(-1.0);
        let t19 = lambert_w(t16 * t17);
        let t20 = t19 + 1.0;
        let t21 = t20 / 2.0;
        let t22 = xc_bessel_I0(t21);
        let t24 = t7 - t11;
        let t25 = 0.1e-9 < t24;
        let t26 = piecewise3(t25, t24, 0.1e-9);
        let t27 = f64::sqrt(t26);
        let t31 = M_SQRT2;
        let t32 = (M_PI * t22 - 4.0 / 3.0 * t13 * t27) * t31;
        let t33 = f64::sqrt(rho[ip]);
        let tvrho0 = -t32 * t33 / 2.0;
        vrho[ip] += tvrho0;
        let t36 = xc_bessel_I1(t21);
        let t37 = M_PI * t36;
        let t40 = 4.0 * tau[ip] * t9;
        let t41 = t2 * t2;
        let t42 = 1.0 / t41;
        let t44 = 3.0 / 4.0 * sigma[ip] * t42;
        let t47 = piecewise3(t15, (-lapl[ip] * t9 + t40 - t44) * t13, 0.0);
        let t49 = 1.0 / t20;
        let t50 = t19 * t49;
        let t51 = 1.0 / t16;
        let t52 = t50 * t51;
        let t56 = t13 / t27;
        let t58 = piecewise3(t25, -t40 + t44, 0.0);
        let t62 = (t37 * t47 * t52 / 2.0 - 2.0 / 3.0 * t56 * t58) * t31;
        let t65 = 1.0 / t33;
        let tv2rho20 = -t62 * t33 / 2.0 - t32 * t65 / 4.0;
        v2rho2[ip] += tv2rho20;
        let t68 = t9 * t13;
        let t70 = piecewise3(t15, t68 / 4.0, 0.0);
        let t71 = t37 * t70;
        let t75 = piecewise3(t25, -t9 / 4.0, 0.0);
        let t79 = (t71 * t52 / 2.0 - 2.0 / 3.0 * t56 * t75) * t31;
        let tv2rhosigma0 = -t79 * t33 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t82 = t3 * t13;
        let t84 = piecewise3(t15, t82 / 2.0, 0.0);
        let t85 = t84 * t19;
        let t86 = t37 * t85;
        let t87 = t49 * t51;
        let t88 = t31 * t33;
        let t89 = t87 * t88;
        let tv2rholapl0 = -t86 * t89 / 4.0;
        v2rholapl[ip] += tv2rholapl0;
        let t93 = piecewise3(t15, -2.0 * t82, 0.0);
        let t94 = t37 * t93;
        let t98 = piecewise3(t25, 2.0 * t3, 0.0);
        let t102 = (t94 * t52 / 2.0 - 2.0 / 3.0 * t56 * t98) * t31;
        let tv2rhotau0 = -t102 * t33 / 2.0;
        v2rhotau[ip] += tv2rhotau0;
    }
}
