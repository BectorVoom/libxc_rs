//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1038/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1038<F: Float>(t375: F, t6125: F, t336: F, t9239: F, t2074: F, t824: F, t328: F, t6552: F, t1: F, t6382: F, t2052: F, t8653: F, t8945: F, t6472: F, t8652: F, t8782: F) -> (F, F, F, F, F, F, F, F) {
    let t20173 = 1.0 / t6125 / t375;
    let t20842 = t9239 * t336;
    let t20898 = t824 * t2074;
    let t21296 = t6552 * t328;
    let t21518 = t6382 * t1;
    let t21884 = t2052 * t2052;
    let t21885 = 1.0 / t21884;
    let t22134 = t8945 * t8653;
    let t22141 = t6472 * t8652;
    let t22142 = t8782 * t22141;
    (t20173, t20842, t20898, t21296, t21518, t21885, t22134, t22142)
}
