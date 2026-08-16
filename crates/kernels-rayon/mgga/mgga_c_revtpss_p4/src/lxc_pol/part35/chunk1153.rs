//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1153/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1153(t29682: f64, t689: f64, t1032: f64, t6041: f64, t867: f64, t786: f64, t18643: f64, t92955: f64, t6037: f64, t92951: f64, t25222: f64, t6030: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105936 = t29682 * t689;
    let t105944 = t6041 * t1032;
    let t105945 = t105944 * t867;
    let t105946 = t786 * t105945;
    let t106006 = t92955 * t18643;
    let t106010 = t92951 * t6037;
    let t106014 = t25222 * t6030;
    (t105936, t105944, t105945, t105946, t106006, t106010, t106014)
}
