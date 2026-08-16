//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2670/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2670(t15707: f64, t15769: f64, t12013: f64, t20029: f64, t1063: f64, t19671: f64, t3172: f64, t19697: f64, t3173: f64, t1041: f64, t19799: f64, t11262: f64, t6301: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65931 = t15707 * t15769;
    let t65960 = t12013 * t20029;
    let t65965 = t1063 * t3172 * t19671;
    let t66003 = t19697 * t3173;
    let t66017 = t1041 * t3172 * t19799;
    let t66022 = t1041 * t11262 * t6301;
    (t65931, t65960, t65965, t66003, t66017, t66022)
}
