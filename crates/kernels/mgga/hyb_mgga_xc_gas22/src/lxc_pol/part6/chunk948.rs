//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 948/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk948<F: Float>(t3474: F, t948: F, t969: F, t2516: F, t3477: F, t1396: F, t2520: F) -> (F, F, F, F) {
    let t9099 = t3474 * t948;
    let t9101 = 2.0 * t9099 * t969;
    let t9103 = 1.0 * t3477 * t2516;
    let t9104 = t1396 * t2520;
    (t9099, t9101, t9103, t9104)
}
