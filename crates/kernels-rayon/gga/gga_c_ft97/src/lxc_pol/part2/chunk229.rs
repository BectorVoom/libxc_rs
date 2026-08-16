//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 229/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk229(t10: f64, t296: f64, t351: f64, t295: f64, t668: f64, t505: f64, t666: f64, t89: f64, t294: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t790 = t10 * t351 * t296;
    let t791 = t790 / 18.0_f64;
    let t792 = t295 * t668;
    let t793 = t792 * t505;
    let t795 = t89 * t666 * t793;
    let t797 = t294 * t294;
    let t798 = 1.0_f64 / t797;
    (t790, t791, t792, t793, t795, t797, t798)
}
