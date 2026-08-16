//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 820/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk820(t18423: f64, t2674: f64, t125: f64, t5977: f64, t221: f64, t2485: f64, t6022: f64, t10850: f64, t14718: f64, t6035: f64, t2662: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18424 = t2674 * t18423;
    let t18426 = t125 * t5977;
    let t18432 = t2485 * t221 * t6022;
    let t18433 = t10850 * t18432;
    let t18440 = t14718 * t6035;
    let t18441 = t2662 * t18440;
    let t18442 = t2661 * t18441;
    (t18424, t18426, t18432, t18433, t18440, t18442)
}
