//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 570/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk570<F: Float>(t2713: F, t2731: F, t2729: F, t2633: F, t2642: F, t2644: F, t2651: F, t2678: F, t2696: F, t2700: F, t2705: F, t2709: F, t2716: F, t2726: F, t493: F, t1052: F, t1101: F) -> (F, F, F, F) {
    let t2732 = t2713 * t2731;
    let t2734 = 0.16081979498692535067e2 * t2729 * t2732;
    let t2735 = t2633 - t2642 - 0.11696447245269292414e1 * t2644 + t2651 - t2678 + 0.19751673498613801407e-1 * t2696 * t493 - 0.36622894612013090108e-3 * t2700 - t2705 + t2709 - t2716 + t2726 + t2734;
    let t2737 = 8.0 * t1052 * t1101;
    (t2732, t2734, t2735, t2737)
}
