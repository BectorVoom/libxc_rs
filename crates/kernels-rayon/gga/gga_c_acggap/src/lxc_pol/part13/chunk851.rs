//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 851/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk851(t2137: f64, t29984: f64, t2140: f64, t2122: f64, t310: f64, t464: f64, t441: f64, t7923: f64, t621: f64, t615: f64, t394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29985 = t2137 * t29984;
    let t29986 = t29985 * t2140;
    let t29988 = t310 * t2122;
    let t29989 = t29988 * t464;
    let t29991 = t7923 * t441;
    let t29992 = t29991 * t621;
    let t29994 = t615 * t29984;
    let t29997 = t394 * t2122;
    (t29986, t29988, t29989, t29992, t29994, t29997)
}
