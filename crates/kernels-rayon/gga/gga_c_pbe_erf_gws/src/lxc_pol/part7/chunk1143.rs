//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1143/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1143(t20480: f64, t6158: f64, t2382: f64, t6677: f64, t3065: f64, t6297: f64, t858: f64, t2306: f64, t6670: f64, t6674: f64, t19561: f64, t20441: f64, t20453: f64, t20459: f64, t20468: f64, t20470: f64, t20475: f64, t2081: f64, t2277: f64, t2307: f64, t3257: f64, t6276: f64, t6579: f64, t6580: f64, t6637: f64, t8944: f64, t904: f64, t9388: f64) -> (f64, f64, f64) {
    let t20481 = t6158 * t20480;
    let t20485 = t2382 * t6677;
    let t20487 = t3065 * t858 * t6297;
    let t20489 = t20485 * t20487 / 8.0_f64;
    let t20490 = t2306 * t6670;
    let t20491 = t2382 * t20490;
    let t20493 = t20491 * t6674 / 4.0_f64;
    let t20494 = 11.0_f64 / 768.0_f64 * t2277 * t3257 * t2081 * t19561 * t20441 + 5.0_f64 / 32.0_f64 * t6579 * t6580 * t2307 - t20453 - t20459 - t6637 * t904 * t8944 * t9388 / 32.0_f64 + t20468 + t6637 * t6276 * t20470 / 128.0_f64 - t6637 * t6276 * t20475 / 64.0_f64 - t6637 * t6276 * t20481 / 96.0_f64 - t20489 + t20493;
    (t20489, t20493, t20494)
}
