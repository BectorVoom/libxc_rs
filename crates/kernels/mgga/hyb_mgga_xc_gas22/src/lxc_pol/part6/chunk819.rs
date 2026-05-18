//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 819/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk819<F: Float>(t1996: F, t550: F, t19: F, t1823: F, t1862: F, t1816: F, t547: F, t126: F, t2986: F, t1874: F, t1877: F, t1815: F, t641: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5861 = t550 * t1996;
    let t5862 = t19 * t5861;
    let t5870 = t550 * t1823;
    let t5871 = t19 * t5870;
    let t5873 = t550 * t1862;
    let t5874 = t19 * t5873;
    let t5876 = t547 * t1816;
    let t5878 = t2986 * t126;
    let t5880 = F::new(5.0) / F::new(288.0) * t19 * t5878;
    let t5881 = t547 * t1874;
    let t5883 = t547 * t1877;
    let t5885 = t1815 * t641;
    (t5861, t5862, t5870, t5871, t5873, t5874, t5876, t5878, t5880, t5881, t5883, t5885)
}
