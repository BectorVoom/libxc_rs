//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 861/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk861(t1053: f64, t363: f64, t2992: f64, t12709: f64, t2983: f64, t12714: f64, t379: f64, t4733: f64, t9144: f64, t4714: f64, t609: f64, t574: f64, t605: f64) -> (f64, f64, f64, f64) {
    let t17375 = t1053 * t363;
    let t17376 = t2992 * t17375;
    let t17377 = t12709 * t17376;
    let t17380 = t2983 * t17375;
    let t17381 = t12714 * t17380;
    let t17384 = t4733 * t379;
    let t17385 = t9144 * t17384;
    let t17388 = t4714 * t609;
    let t17390 = t574 * t605 * t17388;
    (t17377, t17381, t17385, t17390)
}
