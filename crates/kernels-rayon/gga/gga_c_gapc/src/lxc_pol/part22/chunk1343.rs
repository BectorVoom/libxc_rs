//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1343/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1343(t338: f64, t36144: f64, t36158: f64, t36173: f64, t36188: f64, t36204: f64, t36218: f64, t36233: f64, t36248: f64, t12153: f64, t2822: f64, t2469: f64, t3449: f64, t3622: f64) -> (f64, f64, f64) {
    let t36252 = (t36144 + t36158 + t36173 + t36188 + t36204 + t36218 + t36233 + t36248) * t338;
    let t36255 = t12153 * t2822;
    let t36259 = 4.0_f64 * t2469 * t3622 * t3449;
    (t36252, t36255, t36259)
}
