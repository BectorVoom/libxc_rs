//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1063/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1063<F: Float>(t375: F, t6125: F, t336: F, t9239: F, t328: F, t6552: F, t1: F, t6382: F, t2052: F, t3075: F, t837: F, t2306: F, t3074: F, t3039: F, t4384: F, t6792: F) -> (F, F, F, F, F, F, F, F) {
    let t20173 = 1.0 / t6125 / t375;
    let t20842 = t9239 * t336;
    let t21296 = t6552 * t328;
    let t21518 = t6382 * t1;
    let t21884 = t2052 * t2052;
    let t21885 = 1.0 / t21884;
    let t22334 = t3075 * t837;
    let t22336 = t3074 * t2306 * t22334;
    let t22343 = t3039 * t4384;
    let t22379 = t3039 * t6792;
    (t20173, t20842, t21296, t21518, t21885, t22336, t22343, t22379)
}
