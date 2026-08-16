//! GGA_X_2D_PBE fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_pbe_fxc_unpol(
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
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = f64::sqrt(rho[ip]);
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t25 = 0.4604e0 + 0.14106971928508582281e-1 * sigma[ip] / t21;
        let t28 = 0.14604e1 - 0.21196816e0 / t25;
        let t32 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t17 * t18 * t28);
        let tzk0 = 2.0 * t32;
        zk[ip] += tzk0;
        let t38 = t15 * t17;
        let t40 = 1.0 / t18 / t21;
        let t41 = t25 * t25;
        let t42 = 1.0 / t41;
        let t43 = t40 * t42;
        let t48 = piecewise3(t2, 0.0, -t16 * t17 / t18 * t28 / 3.0 + 0.33741119762638214745e-2 * t38 * t43 * sigma[ip]);
        let tvrho0 = 2.0 * rho[ip] * t48 + 2.0 * t32;
        vrho[ip] += tvrho0;
        let t52 = 1.0 / t18 / t20;
        let t56 = piecewise3(t2, 0.0, -0.11247039920879404915e-2 * t38 * t52 * t42);
        let tvsigma0 = 2.0 * rho[ip] * t56;
        vsigma[ip] += tvsigma0;
        let t65 = t20 * t20;
        let t68 = 1.0 / t18 / t65 * t42;
        let t72 = t65 * t21;
        let t76 = 1.0 / t41 / t25;
        let t77 = 1.0 / t18 / t72 * t76;
        let t78 = sigma[ip] * sigma[ip];
        let t83 = piecewise3(t2, 0.0, t16 * t17 / t18 / rho[ip] * t28 / 6.0 - 0.10122335928791464424e-1 * t38 * t68 * sigma[ip] + 0.28559101759679007257e-3 * t38 * t77 * t78);
        let tv2rho20 = 2.0 * rho[ip] * t83 + 4.0 * t48;
        v2rho2[ip] += tv2rho20;
        let t88 = t65 * t20;
        let t90 = 1.0 / t18 / t88;
        let t91 = t90 * t76;
        let t96 = piecewise3(t2, 0.0, 0.28117599802198512288e-2 * t38 * t43 - 0.95197005865596690856e-4 * t38 * t91 * sigma[ip]);
        let tv2rhosigma0 = 2.0 * rho[ip] * t96 + 2.0 * t56;
        v2rhosigma[ip] += tv2rhosigma0;
        let t99 = t65 * rho[ip];
        let t101 = 1.0 / t18 / t99;
        let t105 = piecewise3(t2, 0.0, 0.31732335288532230285e-4 * t38 * t101 * t76);
        let tv2sigma20 = 2.0 * rho[ip] * t105;
        v2sigma2[ip] += tv2sigma20;
    }
}
