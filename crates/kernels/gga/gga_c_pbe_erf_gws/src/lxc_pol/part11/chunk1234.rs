//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1234/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1234<F: Float>(t1123: F, t11700: F, t13544: F, t13593: F, t15150: F, t2118: F, t2157: F, t2255: F, t2277: F, t2312: F, t36612: F, t36920: F, t3826: F, t49491: F, t49498: F, t49500: F, t49507: F, t49508: F, t49514: F, t49521: F, t6275: F, t6637: F, t6685: F, t9499: F) -> F {
    let t49522 = F::new(3.0) / F::new(128.0) * t6685 * t2255 * t1123 * t49491 * t2157 - t49498 + t49500 + t2312 * t11700 * t15150 / F::new(64.0) + t6275 * t3826 * t13544 / F::new(16.0) - t49507 + t6637 * t9499 * t2118 * t49508 / F::new(192.0) + F::new(119.0) / F::new(1152.0) * t36920 - t49514 + t2277 * t2255 * t36612 * t13593 / F::new(256.0) + t49521;
    t49522
}
