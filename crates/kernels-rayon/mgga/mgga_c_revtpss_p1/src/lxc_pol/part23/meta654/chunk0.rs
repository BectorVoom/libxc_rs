//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2382/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2382(t222: f64, t40735: f64, t124: f64, t138: f64, t40649: f64, t9645: f64, t810: f64, t240: f64, t9731: f64, t10760: f64, t2664: f64, t10293: f64, t212: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40737 = 455.0_f64 / 243.0_f64 * t40735 * t222;
    let t40757 = t138 * t124 * t40649 * t9645;
    let t40759 = 0.26776076960158126592e-7_f64 * t40757 * t810;
    let t40763 = t9731 * t240;
    let t40765 = t10760 * t40763 * t2664;
    let t40769 = t800 * t124 * t10293 * t212;
    (t40737, t40757, t40759, t40763, t40765, t40769)
}
