//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 804/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk804(t2251: f64, t2299: f64, t2276: f64, t22: f64, t4258: f64, t191: f64, t369: f64, t371: f64, t364: f64, t56: f64, t6045: f64, t333: f64) -> (f64, f64, f64, f64, f64) {
    let t6578 = t2251 * t2299;
    let t6579 = t2276 * t6578;
    let t6587 = 1.0_f64 / t22 / t4258;
    let t6588 = t6587 * t191;
    let t6589 = t6588 * t369;
    let t6590 = t6589 * t371;
    let t6592 = 595.0_f64 / 10368.0_f64 * t364 * t6590;
    let t6593 = t6045 * t56;
    let t6594 = t6593 * t333;
    (t6579, t6589, t6590, t6592, t6594)
}
