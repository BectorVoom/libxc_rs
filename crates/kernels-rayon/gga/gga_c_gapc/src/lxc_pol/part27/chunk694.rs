//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 694/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk694(t2254: f64, t2546: f64, t147: f64, t2454: f64, t1087: f64, t786: f64, t818: f64, t2716: f64, t918: f64, t2669: f64, t2492: f64, t891: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7708 = t2546 * t2254;
    let t7730 = t2454 * t147;
    let t7735 = t1087 * t786;
    let t7739 = t1087 * t818;
    let t7764 = t918 * t2716;
    let t7776 = t918 * t2669;
    let t7807 = t2492 * t891;
    (t7708, t7730, t7735, t7739, t7764, t7776, t7807)
}
