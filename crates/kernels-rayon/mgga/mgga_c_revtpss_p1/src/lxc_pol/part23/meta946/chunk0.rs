//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3116/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3116(t81904: f64, t81917: f64, t81931: f64, t81944: f64, t81957: f64, t81969: f64, t81983: f64, t81995: f64, t1180: f64, t1187: f64, t1188: f64, t12553: f64, t17023: f64, t17032: f64, t20537: f64, t20615: f64, t20619: f64, t20678: f64, t24375: f64, t24376: f64, t24408: f64, t3491: f64, t45064: f64, t45188: f64, t45190: f64, t5158: f64, t5180: f64, t58242: f64, t6538: f64, t81591: f64, t81593: f64, t81596: f64, t81599: f64, t81601: f64, t81604: f64) -> (f64, f64) {
    let t81998 = t81904 + t81917 + t81931 + t81944 + t81957 + t81969 + t81983 + t81995;
    let t82006 = 0.30762056574649219974e4_f64 * t12553 * t20678 * t5180 + 0.91082604192152556044e5_f64 * t45188 * t24375 * t45190 * t1187 - t81591 + t81593 + t81596 - t81599 + t81601 + t81604 + 0.17544670867903938621e1_f64 * t5158 * t20537 + 0.51947577317044391276e2_f64 * t58242 * t6538 - 0.10389515463408878255e3_f64 * t45064 * t24376 + 0.5848223622634646207e0_f64 * t3491 * t24408 + 0.5848223622634646207e0_f64 * t1180 * t81998 * t1188 - 6.0_f64 * t17023 * t20615 + 0.96491876992155210402e2_f64 * t17032 * t20619;
    (t81998, t82006)
}
