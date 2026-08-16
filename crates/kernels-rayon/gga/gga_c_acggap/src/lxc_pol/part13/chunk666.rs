//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 666/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk666(t4128: f64, t5336: f64, t5357: f64, t5390: f64, t1662: f64, t814: f64, t467: f64, t495: f64, t406: f64, t513: f64, t157: f64, t506: f64) -> (f64, f64, f64, f64, f64) {
    let t5392 = t4128 + t5336 + t5357 + t5390;
    let t5399 = t1662 * t814;
    let t5439 = t495 * t467;
    let t5605 = t513 * t406;
    let t5606 = t5605 * t157;
    let t5615 = t506 * t406;
    (t5392, t5399, t5439, t5606, t5615)
}
