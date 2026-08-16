//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1364/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1364(t42872: f64, t43351: f64, t1035: f64, t42859: f64, t342: f64, t357: f64, t3057: f64, t4980: f64, t11200: f64, t3286: f64, t4995: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43352 = t43351 * t42872;
    let t43400 = t42859 * t1035;
    let t43401 = t342 * t43400;
    let t43402 = t43351 * t357;
    let t43438 = t3057 * t4980;
    let t43446 = t11200 * t3286;
    let t43456 = t3057 * t4995;
    let t43471 = t42859 * t3143;
    (t43352, t43401, t43402, t43438, t43446, t43456, t43471)
}
