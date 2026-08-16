//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1469/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1469(t2632: f64, t4233: f64, t4180: f64, t4181: f64, t2639: f64, t5619: f64, t5614: f64, t1484: f64, t4119: f64) -> (f64, f64, f64, f64, f64) {
    let t16935 = t2632 * t4233;
    let t16937 = t4180 * t4181 * t16935;
    let t16940 = t2639 * t5619;
    let t16942 = t2639 * t5614;
    let t16944 = t1484 * t4119;
    (t16935, t16937, t16940, t16942, t16944)
}
