//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 873/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk873<F: Float>(t9378: F, t9388: F, t684: F, t664: F, t3554: F, t5771: F, t2860: F, t2875: F, t3591: F, t5498: F, t2874: F, t730: F) -> (F, F, F, F, F, F, F, F) {
    let t9389 = t9378 + t9388;
    let t9390 = t9389 * t684;
    let t9392 = F::new(1.0) * t664 * t9390;
    let t9394 = F::cast_from(0.16081979498692535067e2_f64) * t5771 * t3554;
    let t9396 = F::cast_from(0.34631718211362927517e2_f64) * t2860 * t2875;
    let t9397 = t5498 * t3591;
    let t9398 = t9397 * t2874;
    let t9400 = F::cast_from(0.10389515463408878255e3_f64) * t730 * t9398;
    (t9389, t9390, t9392, t9394, t9396, t9397, t9398, t9400)
}
