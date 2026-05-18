//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 258/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk258<F: Float>(t769: F, t896: F, t334: F, t317: F, t19: F, t328: F, t275: F, t308: F) -> (F, F, F, F, F, F, F) {
    let t897 = t896 * t769;
    let t906 = t334 * t334;
    let t907 = F::new(1.0) / t906;
    let t908 = t317 * t907;
    let t909 = t19 * t328;
    let t910 = t308 * t275;
    let t911 = F::new(1.0) / t910;
    (t897, t906, t907, t908, t909, t910, t911)
}
