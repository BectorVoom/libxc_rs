//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1017/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1017<F: Float>(t20485: F, t20487: F, t2306: F, t6670: F, t2382: F, t6674: F, t19561: F, t20441: F, t20453: F, t20459: F, t20468: F, t20470: F, t20475: F, t20481: F, t2081: F, t2277: F, t2307: F, t3257: F, t6276: F, t6579: F, t6580: F, t6637: F, t8944: F, t904: F, t9388: F) -> (F, F, F) {
    let t20489 = t20485 * t20487 / 8.0;
    let t20490 = t2306 * t6670;
    let t20491 = t2382 * t20490;
    let t20493 = t20491 * t6674 / 4.0;
    let t20494 = 11.0 / 768.0 * t2277 * t3257 * t2081 * t19561 * t20441 + 5.0 / 32.0 * t6579 * t6580 * t2307 - t20453 - t20459 - t6637 * t904 * t8944 * t9388 / 32.0 + t20468 + t6637 * t6276 * t20470 / 128.0 - t6637 * t6276 * t20475 / 64.0 - t6637 * t6276 * t20481 / 96.0 - t20489 + t20493;
    (t20489, t20493, t20494)
}
