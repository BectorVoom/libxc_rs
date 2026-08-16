//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 960/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk960<F: Float>(t7923: F, t7927: F, t7931: F, t7934: F, t7939: F, t7943: F, t7944: F, t7947: F, t7949: F, t7953: F, t7955: F, t7958: F, t7961: F, t7965: F, t7968: F, t7970: F, t7971: F) -> F {
    let t8456 = -t7923 + t7927 - t7931 + t7934 - t7939 - t7943 - t7944 - t7947 - t7949 + t7953 + t7955 - t7958 - t7961 + t7965 + t7968 + t7970 + t7971;
    t8456
}
