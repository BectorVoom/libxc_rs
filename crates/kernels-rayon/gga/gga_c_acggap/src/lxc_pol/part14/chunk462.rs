//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 462/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk462(t2138: f64, t2140: f64, t441: f64, t615: f64, t2130: f64) -> (f64, f64, f64) {
    let t2142 = 0.8673628188205199462e0_f64 * t2138 * t2140;
    let t2143 = t615 * t441;
    let t2146 = t615 * t2130;
    (t2142, t2143, t2146)
}
