//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 839/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk839<F: Float>(t10195: F, t275: F, t30400: F, t3351: F, t3352: F, t515: F, t2286: F, t38351: F, t38530: F, t9171: F, t14243: F, t16503: F, t552: F, t9157: F, t1364: F, t1635: F, t2024: F, t2402: F, t30311: F, t35327: F, t39786: F, t39789: F, t39792: F, t39797: F, t39801: F, t39804: F, t39809: F, t39827: F, t5898: F, t8800: F, t884: F) -> (F,) {
    let t45798 = t275 * t10195;
    let t45811 = t3351 * t3352 * t515 * t30400;
    let t45813 = t38351 * t2286;
    let t45818 = t38530 * t9171;
    let t45822 = t16503 * t14243 * t552 * t9157;
    let t45824 = t45798 - 0.33105799917009430643e-4 * t35327 - t39786 - 0.30487649791575028314e-3 * t39789 - 0.3903207359137154578e-3 * t39792 - t39797 - t39801 - 0.30487649791575028314e-3 * t39804 + t39809 - 0.47896966807455234256e0 * t1364 * t2402 * t1635 - 0.23948483403727617128e0 * t884 * t8800 * t5898 - 0.25538759935978703638e-4 * t45811 + 0.25538759935978703638e-4 * t45813 - 0.11974241701863808564e0 * t884 * t2024 * t30311 - t39827 + 0.85129199786595678796e-5 * t45818 + 0.76616279807936110914e-4 * t45822;
    (t45824,)
}
