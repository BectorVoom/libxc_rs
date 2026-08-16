//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3093/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3093(t1196: f64, t20891: f64, t24375: f64, t43752: f64, t16840: f64, t20574: f64, t17092: f64, t20577: f64, t1149: f64, t12248: f64, t24221: f64, t3433: f64, t5104: f64, t6439: f64) -> (f64, f64, f64, f64, f64) {
    let t81589 = 0.12304822629859687989e5_f64 * t1196 * t43752 * t24375 * t20891;
    let t81591 = 18.0_f64 * t16840 * t20574;
    let t81593 = 12.0_f64 * t17092 * t20577;
    let t81596 = 24.0_f64 * t12248 * t24221 * t1149;
    let t81599 = 18.0_f64 * t3433 * t6439 * t5104;
    (t81589, t81591, t81593, t81596, t81599)
}
