//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 314/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk314<F: Float>(t810: F, t904: F, t933: F, t862: F, t879: F, t882: F, t890: F, t902: F, t907: F, t914: F, t918: F, t927: F, t929: F) -> (F, F) {
    let t935 = t933 * t904 * t810;
    let t938 = t862 - t879 - t882 - t890 + t902 * t907 / 1536.0 - t914 * t918 / 1536.0 - t927 - t929 * t935 / 768.0;
    (t935, t938)
}
