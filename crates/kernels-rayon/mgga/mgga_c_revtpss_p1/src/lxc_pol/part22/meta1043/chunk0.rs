//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3646/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3646(t1196: f64, t20890: f64, t3524: f64, t16655: f64, t17092: f64, t16658: f64, t58342: f64, t16665: f64, t16840: f64, t16669: f64, t58473: f64, t16784: f64, t5202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68959 = 0.6233709278045326953e3_f64 * t1196 * t20890 * t3524;
    let t68961 = 4.0_f64 * t17092 * t16655;
    let t68963 = 0.19298375398431042081e3_f64 * t58342 * t16658;
    let t68965 = 0.32163958997385070134e2_f64 * t16840 * t16665;
    let t68967 = 0.1034520258385468006e4_f64 * t58473 * t16669;
    let t68969 = 0.23392894490538584828e1_f64 * t16784 * t5202;
    (t68959, t68961, t68963, t68965, t68967, t68969)
}
