//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 867/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk867(t2611: f64, t6002: f64, t2498: f64, t2518: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2621: f64, t2628: f64, t2632: f64, t5924: f64, t5925: f64, t5927: f64, t5943: f64, t5945: f64, t5947: f64, t5948: f64, t6001: f64) -> (f64, f64) {
    let t6004 = 12.0_f64 * t2611 * t6002;
    let t6005 = -t2498 - t2518 - t2522 + t5947 + t2610 + t2579 + t2587 + t6001 - t2562 + t5925 - t2569 + t2621 + t2628 + t2632 + t6004 + t5943 + t5945 - t5924 - t5948 + t5927;
    (t6004, t6005)
}
