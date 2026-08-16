//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3807/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3807(t18123: f64, t20692: f64, t3794: f64, t5023: f64, t5505: f64, t68942: f64, t68946: f64, t68949: f64, t68951: f64, t68954: f64, t68956: f64, t68959: f64, t68961: f64, t68963: f64, t68965: f64, t68967: f64, t68969: f64) -> f64 {
    let t73283 = -2.0_f64 * t18123 * t5023 * t5505 - t20692 * t3794 * t5023 + t68942 + t68946 + t68949 + t68951 + t68954 + t68956 - t68959 - t68961 - t68963 + t68965 + t68967 - t68969;
    t73283
}
