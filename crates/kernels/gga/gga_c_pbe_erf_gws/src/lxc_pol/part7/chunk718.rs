//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 718/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk718<F: Float>(t343: F, t6231: F, t904: F, t916: F, t2277: F, t2312: F, t6182: F, t6186: F, t6190: F, t6192: F, t6198: F, t6204: F, t6208: F, t6213: F, t6219: F, t6224: F, t6225: F, t6230: F, t914: F) -> (F, F, F) {
    let t6232 = t6231 * t343;
    let t6234 = t916 * t904 * t6232;
    let t6237 = -t6182 + t6186 - t6190 - t2312 * t6192 / 128.0 - t2277 * t6198 / 256.0 + 7.0 / 96.0 * t6204 - t2312 * t6208 / 128.0 - t2312 * t6213 / 128.0 - t6219 + t6224 - 7.0 / 768.0 * t6225 - t6230 - t914 * t6234 / 1536.0;
    (t6232, t6234, t6237)
}
