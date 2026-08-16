//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1466/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1466(t41308: f64, t41312: f64, t41320: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41433: f64, t41436: f64, t41439: f64, t41441: f64) -> f64 {
    let t41732 = -0.41318e1_f64 * t41365 + 0.13772666666666666667e1_f64 * t41367 + 0.41318e1_f64 * t41308 + 0.123954e2_f64 * t41312 + 0.309885e1_f64 * t41320 - 0.103295e1_f64 * t41327 - 0.13772666666666666666e1_f64 * t41330 - 0.91817777777777777776e0_f64 * t41332 + 0.68863333333333333332e0_f64 * t41334 + 0.76514814814814814814e0_f64 * t41336 - 0.104195e0_f64 * t41433 + 0.250068e1_f64 * t41436 + 0.62517e0_f64 * t41439 + 0.12349037037037037037e1_f64 * t41441;
    t41732
}
