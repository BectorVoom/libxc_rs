//! MGGA_X_2D_PRHG07 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_prhg07.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::bessel::{xc_bessel_I0, xc_bessel_I1};
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_2d_prhg07_vxc_unpol(
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
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5::<f64>(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = f64::sqrt(zeta_threshold);
        let t12 = f64::sqrt(t8);
        let t14 = piecewise3::<f64>(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = M_PI * t14;
        let t16 = M_SQRT2;
        let t17 = f64::sqrt(rho[ip]);
        let t18 = t16 * t17;
        let t19 = rho[ip] * rho[ip];
        let t20 = 1.0 / t19;
        let t25 = t19 * rho[ip];
        let t26 = 1.0 / t25;
        let t30 = 1.0 / M_PI;
        let t31 = (lapl[ip] * t20 / 2.0 - 2.0 * tau[ip] * t20 + sigma[ip] * t26 / 4.0) * t30;
        let t32 = -0.9999999999e0 < t31;
        let t33 = piecewise3::<f64>(t32, t31, -0.9999999999e0);
        let t34 = f64::exp(-1.0);
        let t36 = lambert_w::<f64>(t33 * t34);
        let t37 = t36 + 1.0;
        let t38 = t37 / 2.0;
        let t39 = xc_bessel_I0::<f64>(t38);
        let t43 = piecewise3::<f64>(t3, 0.0, -t15 * t18 * t39 / 8.0);
        let tzk0 = 2.0 * t43;
        zk[ip] += tzk0;
        let t45 = t16 / t17;
        let t48 = t15 * t18;
        let t49 = xc_bessel_I1::<f64>(t38);
        let t53 = t19 * t19;
        let t54 = 1.0 / t53;
        let t59 = piecewise3::<f64>(t32, (-lapl[ip] * t26 + 4.0 * tau[ip] * t26 - 3.0 / 4.0 * sigma[ip] * t54) * t30, 0.0);
        let t61 = 1.0 / t37;
        let t62 = t36 * t61;
        let t64 = t62 / t33;
        let t65 = t49 * t59 * t64;
        let t69 = piecewise3::<f64>(t3, 0.0, -t15 * t45 * t39 / 16.0 - t48 * t65 / 16.0);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t43;
        vrho[ip] += tvrho0;
        let t72 = t26 * t30;
        let t74 = piecewise3::<f64>(t32, t72 / 4.0, 0.0);
        let t75 = t49 * t74;
        let t76 = t75 * t64;
        let t79 = piecewise3::<f64>(t3, 0.0, -t48 * t76 / 16.0);
        let tvsigma0 = 2.0 * rho[ip] * t79;
        vsigma[ip] += tvsigma0;
        let t81 = t20 * t30;
        let t83 = piecewise3::<f64>(t32, t81 / 2.0, 0.0);
        let t84 = t49 * t83;
        let t85 = t84 * t64;
        let t88 = piecewise3::<f64>(t3, 0.0, -t48 * t85 / 16.0);
        let tvlapl0 = 2.0 * rho[ip] * t88;
        vlapl[ip] += tvlapl0;
        let t91 = piecewise3::<f64>(t32, -2.0 * t81, 0.0);
        let t92 = t49 * t91;
        let t93 = t92 * t64;
        let t96 = piecewise3::<f64>(t3, 0.0, -t48 * t93 / 16.0);
        let tvtau0 = 2.0 * rho[ip] * t96;
        vtau[ip] += tvtau0;
    }
}
