//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1143/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1143<F: Float>(t20480: F, t6158: F, t2382: F, t6677: F, t3065: F, t6297: F, t858: F, t2306: F, t6670: F, t6674: F, t19561: F, t20441: F, t20453: F, t20459: F, t20468: F, t20470: F, t20475: F, t2081: F, t2277: F, t2307: F, t3257: F, t6276: F, t6579: F, t6580: F, t6637: F, t8944: F, t904: F, t9388: F) -> (F, F, F) {
    let t20481 = t6158 * t20480;
    let t20485 = t2382 * t6677;
    let t20487 = t3065 * t858 * t6297;
    let t20489 = t20485 * t20487 / F::cast_from(8.0_f64);
    let t20490 = t2306 * t6670;
    let t20491 = t2382 * t20490;
    let t20493 = t20491 * t6674 / F::cast_from(4.0_f64);
    let t20494 = F::cast_from(11.0_f64) / F::cast_from(768.0_f64) * t2277 * t3257 * t2081 * t19561 * t20441 + F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t6579 * t6580 * t2307 - t20453 - t20459 - t6637 * t904 * t8944 * t9388 / F::cast_from(32.0_f64) + t20468 + t6637 * t6276 * t20470 / F::cast_from(128.0_f64) - t6637 * t6276 * t20475 / F::cast_from(64.0_f64) - t6637 * t6276 * t20481 / F::cast_from(96.0_f64) - t20489 + t20493;
    (t20489, t20493, t20494)
}
