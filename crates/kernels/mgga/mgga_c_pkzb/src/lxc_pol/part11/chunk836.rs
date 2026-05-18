//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 836/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk836<F: Float>(t2593: F, t2639: F, t179: F, t3403: F, t5221: F, t1702: F, t3407: F, t3402: F, t568: F, t581: F, t1024: F, t2575: F) -> (F, F, F, F, F, F) {
    let t8920 = t2593 * t2639;
    let t8921 = t179 * t8920;
    let t8924 = t5221 * t3403;
    let t8926 = t1702 * t3407;
    let t8931 = t581 * t3402 * t568;
    let t8935 = t581 * t1024 * t2575;
    (t8920, t8921, t8924, t8926, t8931, t8935)
}
