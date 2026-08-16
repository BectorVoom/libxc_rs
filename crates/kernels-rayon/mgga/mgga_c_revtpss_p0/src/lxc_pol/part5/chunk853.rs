//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 853/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk853(t1583: f64, t198: f64, t207: f64, t2393: f64, t2403: f64, t2411: f64, t2621: f64, t5927: f64, t5943: f64, t5945: f64, t5947: f64, t5948: f64, t5962: f64, t5966: f64, t5970: f64, t6001: f64, t6004: f64, t6075: f64, t765: f64, t892: f64) -> (f64, f64) {
    let t6079 = t1583 * t1583;
    let t6083 = -t198 * t207 * t2411 * t6079 + t198 * t207 * t6075 * t892 + 6.0_f64 * t198 * t2393 * t5966 + 3.0_f64 * t198 * t5962 * t765 + 6.0_f64 * t2403 * t5970 + t2621 + t5927 + t5943 + t5945 + t5947 - t5948 + t6001 + t6004;
    (t6079, t6083)
}
