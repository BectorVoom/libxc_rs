//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 903/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk903(t429: f64, t7457: f64, t7458: f64, t7459: f64, t3378: f64, t7432: f64, t2074: f64, t12726: f64, t2067: f64, t2070: f64, t1190: f64, t30644: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30790 = t7457 * t7458 * t429 * t7459;
    let t30792 = t3378 * t7432;
    let t30793 = t30792 * t2074;
    let t30797 = t12726 * t2067;
    let t30798 = t30797 * t2070;
    let t30800 = t30644 * t1190;
    (t30790, t30792, t30793, t30797, t30798, t30800)
}
