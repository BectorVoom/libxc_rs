//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 675/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk675(t10: f64, t296: f64, t3050: f64, t1636: f64, t825: f64, t89: f64, t2404: f64, t798: f64, t2770: f64, t863: f64, t848: f64, t2344: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10397 = t10 * t3050 * t296;
    let t10398 = 14.0_f64 / 81.0_f64 * t10397;
    let t10400 = t89 * t1636 * t825;
    let t10409 = t2404 * t798;
    let t10443 = t2770 * t863;
    let t10447 = t848 * t863;
    let t10478 = t2344 * t798;
    (t10397, t10398, t10400, t10409, t10443, t10447, t10478)
}
