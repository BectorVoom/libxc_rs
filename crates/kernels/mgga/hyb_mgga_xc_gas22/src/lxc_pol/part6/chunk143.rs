//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 143/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk143<F: Float>(t439: F, t14: F, t237: F, t240: F, t442: F) -> (F, F, F, F, F) {
    let t445 = pow_3_2(t439);
    let t448 = t237 * t14 * t240;
    let t450 = 0.379785e1 * t442 + 0.8969e0 * t439 + 0.204775e0 * t445 + 0.123235e0 * t448;
    let t453 = 1.0 + 0.16081979498692535067e2 / t450;
    let t454 = f64::ln(t453);
    (t445, t448, t450, t453, t454)
}
