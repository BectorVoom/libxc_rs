//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1793/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1793(t40067: f64, t40072: f64, t4140: f64, t47100: f64, t47102: f64, t47107: f64, t47109: f64, t47111: f64, t47114: f64, t47116: f64, t47118: f64, t47120: f64, t47122: f64, t47124: f64, t47126: f64, t5536: f64, t9984: f64) -> f64 {
    let t47681 = 72.0_f64 * t4140 * t5536 * t9984 + t40067 - t40072 - t47100 - t47102 - t47107 - t47109 - t47111 + t47114 + t47116 - t47118 - t47120 + t47122 + t47124 + t47126;
    t47681
}
