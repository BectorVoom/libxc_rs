//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1936/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1936(t18471: f64, t25270: f64, t18446: f64, t18629: f64, t18428: f64, t27261: f64, t18651: f64, t18639: f64, t18643: f64, t92955: f64, t18456: f64, t6037: f64, t92951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t105993 = t25270 * t18471;
    let t105995 = t25270 * t18446;
    let t105997 = t25270 * t18629;
    let t105999 = t27261 * t18428;
    let t106001 = t25270 * t18651;
    let t106003 = t25270 * t18639;
    let t106006 = t92955 * t18643;
    let t106008 = t27261 * t18456;
    let t106010 = t92951 * t6037;
    (t105993, t105995, t105997, t105999, t106001, t106003, t106006, t106008, t106010)
}
