//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 549/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk549(t1528: f64, t920: f64, t72: f64, t942: f64, t1524: f64, t1526: f64, t1527: f64, t342: f64, t343: f64, t948: f64, t947: f64) -> (f64, f64, f64, f64, f64) {
    let t4406 = t1528 * t920;
    let t4410 = t72 * t942;
    let t4414 = t948 - t1524 - t1526 * t1527 * t4406 / 12.0_f64 - t342 * t343 * t4410 / 4.0_f64;
    let t4415 = t4414 * t947;
    let t4417 = t920 * t920;
    (t4406, t4410, t4414, t4415, t4417)
}
