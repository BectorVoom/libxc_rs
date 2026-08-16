//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 756/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk756(t33: f64, t265: f64, t502: f64, t1711: f64, t1940: f64, t1963: f64, t2403: f64, t7091: f64, t7783: f64, t7863: f64, t7869: f64, t7855: f64, t1469: f64, t2003: f64, t57: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t7876 = 3.0_f64 / 2.0_f64 * t2403 * t7863 + t1940 * t7783 * t33 / 2.0_f64 - t1940 * t7091 * t7869 / 2.0_f64 + t1940 * t1963 * t1711 / 2.0_f64;
    let t7877 = piecewise3(t503, 0.0_f64, t7855);
    let t7882 = piecewise3(t400, t7876, -t2003 * t1469 / 2.0_f64 + t7877 * t57 / 2.0_f64);
    (t7877, t7882)
}
