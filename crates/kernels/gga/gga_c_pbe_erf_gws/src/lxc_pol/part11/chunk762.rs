//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 762/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk762<F: Float>(t12374: F, t12375: F, t12377: F, t12378: F, t4826: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4854: F, t4856: F, t4858: F, t4861: F, t4864: F, t13149: F, t13151: F, t13153: F) -> (F,) {
    let t13154 = -t12374 - t12375 - t12377 + t12378 + t4826 - t4837 - t4840 - t4843 + t4846 + t4849 + t4854 - t4856 - t4858 - t4861 - t4864;
    let t13156 = t13149 + t13151 + t13153 + t13154;
    (t13156,)
}
