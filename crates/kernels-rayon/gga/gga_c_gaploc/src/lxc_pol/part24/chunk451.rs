//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 451/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk451(t213: f64, t218: f64, t1654: f64, t847: f64, t211: f64, t215: f64, t408: f64, t608: f64, t1666: f64, t851: f64, t220: f64, t612: f64, t43: f64, t448: f64, t894: f64, zeta_threshold: f64) -> (f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2214 = t1654 * t847;
    let t2217 = t215 * t211;
    let t2221 = piecewise3(t214, 0.0_f64, 4.0_f64 / 9.0_f64 * t2214 * t608 + 8.0_f64 / 3.0_f64 * t2217 * t408);
    let t2222 = t1666 * t851;
    let t2225 = t220 * t211;
    let t2229 = piecewise3(t219, 0.0_f64, 4.0_f64 / 9.0_f64 * t2222 * t612 - 8.0_f64 / 3.0_f64 * t2225 * t408);
    let t2231 = (t2221 + t2229) * t43;
    let t2264 = t894 * t448;
    (t2231, t2264)
}
