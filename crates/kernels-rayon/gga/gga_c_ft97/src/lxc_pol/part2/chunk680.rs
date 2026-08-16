//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 680/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk680(t2404: f64, t798: f64, t2770: f64, t863: f64, t848: f64, t2878: f64, t8392: f64, t2885: f64, t2344: f64, t309: f64, t2680: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10409 = t2404 * t798;
    let t10443 = t2770 * t863;
    let t10447 = t848 * t863;
    let t10461 = t8392 * t2878;
    let t10463 = t8392 * t2885;
    let t10478 = t2344 * t798;
    let t10479 = t10478 * t309;
    let t10491 = t665 * t2680;
    (t10409, t10443, t10447, t10461, t10463, t10478, t10479, t10491)
}
