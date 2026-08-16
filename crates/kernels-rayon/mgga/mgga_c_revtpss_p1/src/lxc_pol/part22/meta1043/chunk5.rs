//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3651/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3651(t448: f64, t68983: f64, t68997: f64, t69011: f64, t69025: f64, t300: f64, t68946: f64, t68949: f64, t68951: f64, t68954: f64, t68956: f64, t68959: f64, t68961: f64, t68963: f64, t68965: f64, t68967: f64, t68969: f64, t68971: f64) -> (f64, f64, f64) {
    let t69028 = (t68983 + t68997 + t69011 + t69025) * t448;
    let t69030 = 0.19751673498613801407e-1_f64 * t300 * t69028;
    let t69031 = t68946 + t68949 + t68951 + t68954 + t68956 - t68959 - t68961 - t68963 + t68965 + t68967 - t68969 - t68971 + t69030;
    (t69028, t69030, t69031)
}
