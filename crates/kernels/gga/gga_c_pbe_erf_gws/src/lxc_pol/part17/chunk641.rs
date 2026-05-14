//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 641/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk641<F: Float>(t2345: F, t3240: F, t875: F, t2253: F, t2343: F, t3125: F, t3130: F, t3136: F, t3144: F, t3150: F, t3176: F, t3224: F, t3228: F, t3232: F, t3237: F, t2264: F, t367: F, t899: F) -> (F, F, F) {
    let t3242 = t2345 * t3240 * t875;
    let t3245 = -t2253 * t3224 / 768.0 + t3144 + t3150 - t3125 + t3176 - t3136 - t2253 * t3228 / 768.0 - t3130 + t2343 * t3232 / 384.0 - t2343 * t3237 / 1536.0 + t2343 * t3242 / 384.0;
    let t3247 = t899 * t2264 * t367;
    (t3242, t3245, t3247)
}
