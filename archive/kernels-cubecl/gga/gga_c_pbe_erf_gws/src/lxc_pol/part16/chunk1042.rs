//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1042/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1042<F: Float>(t3219: F, t3235: F, t8904: F, t2277: F, t8894: F, t8899: F, t8901: F, t8908: F, t8912: F, t8917: F, t8923: F, t902: F, t9411: F, t9415: F, t9417: F, t9421: F, t9425: F) -> (F, F) {
    let t9427 = t3235 * t3219 * t8904;
    let t9430 = -t8894 + t902 * t9411 / F::cast_from(1536.0_f64) + t8899 - t9415 - t8901 + t902 * t9417 / F::cast_from(768.0_f64) - t8908 + t8912 - t2277 * t9421 / F::cast_from(1536.0_f64) + t8917 - t9425 * t9427 / F::cast_from(128.0_f64) + t8923;
    (t9427, t9430)
}
