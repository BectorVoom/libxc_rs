//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 893/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk893<F: Float>(t1114: F, t3747: F, t6670: F, t3871: F, t6505: F, t20948: F, t3754: F, t28195: F, t3128: F, t11412: F, t904: F, t3805: F, t6616: F, t3717: F, t3808: F, t11609: F, t2306: F, t360: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37286 = t1114 * t3747 * t6670;
    let t37363 = t6505 * t3871;
    let t37377 = t20948 * t3754;
    let t37380 = t3128 * t28195;
    let t37396 = t904 * t11412;
    let t37507 = t3805 * t6616;
    let t37632 = t3717 * param_a_c;
    let t37645 = t904 * t3808;
    let t37701 = t2306 * t11609 * t360;
    (t37286, t37363, t37377, t37380, t37396, t37507, t37632, t37645, t37701)
}
