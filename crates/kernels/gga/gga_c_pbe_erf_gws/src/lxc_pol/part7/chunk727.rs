//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 727/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk727<F: Float>(t5899: F, t5914: F, t5916: F, t5923: F, t5930: F, t5956: F, t5958: F, t5959: F, t163: F, t169: F, t684: F, t784: F) -> (F, F) {
    let t5962 = t5899 + t5914 + t5916 + t5923 + t5930 + t5956 + t5958 + t5959;
    let t5969 = t169 * t784 * t684 * t163;
    (t5962, t5969)
}
