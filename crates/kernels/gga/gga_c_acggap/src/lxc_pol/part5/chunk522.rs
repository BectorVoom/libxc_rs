//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 522/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk522<F: Float>(t2838: F, t286: F, t229: F, t699: F, t224: F, t902: F, t277: F, t715: F, t192: F, t839: F, t43: F, t98: F) -> (F, F, F, F, F, F, F, F) {
    let t2839 = t286 * t2838;
    let t2840 = F::cast_from(0.10389515463408878255e3_f64) * t2839;
    let t2841 = t229 * t699;
    let t2843 = t224 * t902;
    let t2845 = t229 * t902;
    let t2847 = t715 * t277;
    let t2853 = t192 * t839;
    let t2861 = F::cast_from(1.0_f64) / t98 / t43;
    (t2839, t2840, t2841, t2843, t2845, t2847, t2853, t2861)
}
