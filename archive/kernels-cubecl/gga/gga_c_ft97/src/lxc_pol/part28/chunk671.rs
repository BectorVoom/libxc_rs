//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 671/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk671<F: Float>(t12411: F, t5784: F, t1013: F, t58: F, t538: F, t22591: F, t1008: F, t554: F, t22767: F, t6604: F, t22632: F, t5813: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26729 = t12411 * t5784;
    let t26738 = t58 * t1013;
    let t26739 = t26738 * t538;
    let t26740 = t22591 * t26739;
    let t26743 = t58 * t1008;
    let t26744 = t26743 * t554;
    let t26745 = t22591 * t26744;
    let t26750 = t26738 * t554;
    let t26751 = t22591 * t26750;
    let t26759 = t22767 * t6604;
    let t26762 = t22632 * t6604;
    let t26763 = t5813 * t26762;
    (t26729, t26739, t26740, t26744, t26745, t26750, t26751, t26759, t26763)
}
