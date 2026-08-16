//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 145/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk145(t439: f64, t14: f64, t237: f64, t240: f64, t442: f64) -> (f64, f64, f64, f64, f64) {
    let t445 = pow_3_2(t439);
    let t448 = t237 * t14 * t240;
    let t450 = 0.379785e1_f64 * t442 + 0.8969e0_f64 * t439 + 0.204775e0_f64 * t445 + 0.123235e0_f64 * t448;
    let t453 = 1.0_f64 + 0.16081979498692535067e2_f64 / t450;
    let t454 = f64::ln(t453);
    (t445, t448, t450, t453, t454)
}
