//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 820/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk820<F: Float>(t1801: F, t224: F, t6527: F, t2232: F, t786: F, t230: F) -> (F, F, F, F) {
    let t6536 = 1.0 / t224 / t1801;
    let t6552 = 0.28842592592592592592e-1 * t6527;
    let t6561 = 1.0 / t2232 / t786;
    let t6562 = t230 * t6561;
    (t6536, t6552, t6561, t6562)
}
