//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1340/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1340<F: Float>(t11961: F, t14011: F, t11635: F, t54279: F, t14024: F, t3783: F, t11640: F, t14498: F, t11819: F, t338: F, t54055: F, t11589: F, t4039: F) -> (F, F, F, F, F, F) {
    let t57108 = t14011 * t11961;
    let t57110 = t54279 * t11635;
    let t57112 = t3783 * t14024;
    let t57114 = t14498 * t11640;
    let t57117 = t54055 * t338 * t11819;
    let t57119 = t4039 * t11589;
    (t57108, t57110, t57112, t57114, t57117, t57119)
}
