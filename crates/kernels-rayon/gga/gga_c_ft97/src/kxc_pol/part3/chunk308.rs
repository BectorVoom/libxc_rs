//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 308/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk308(t358: f64, t81: f64, t363: f64, t432: f64, t72: f64, t1524: f64, t1526: f64, t1527: f64, t342: f64, t343: f64, t438: f64, t14: f64, t360: f64) -> (f64, f64, f64, f64, f64) {
    let t1528 = t81 * t358;
    let t1529 = t1528 * t363;
    let t1533 = t72 * t432;
    let t1537 = t438 - t1524 - t1526 * t1527 * t1529 / 12.0_f64 - t342 * t343 * t1533 / 4.0_f64;
    let t1541 = 1.0_f64 / t14 / t360;
    (t1528, t1529, t1533, t1537, t1541)
}
