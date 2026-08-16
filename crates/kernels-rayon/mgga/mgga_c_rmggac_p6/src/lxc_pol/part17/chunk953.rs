//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 953/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk953(t1364: f64, t1635: f64, t2024: f64, t2402: f64, t30311: f64, t35327: f64, t39786: f64, t39789: f64, t39792: f64, t39797: f64, t39801: f64, t39804: f64, t39809: f64, t39827: f64, t45798: f64, t45811: f64, t45813: f64, t45818: f64, t45822: f64, t5898: f64, t8800: f64, t884: f64) -> f64 {
    let t45824 = t45798 - 0.33105799917009430643e-4_f64 * t35327 - t39786 - 0.30487649791575028314e-3_f64 * t39789 - 0.3903207359137154578e-3_f64 * t39792 - t39797 - t39801 - 0.30487649791575028314e-3_f64 * t39804 + t39809 - 0.47896966807455234256e0_f64 * t1364 * t2402 * t1635 - 0.23948483403727617128e0_f64 * t884 * t8800 * t5898 - 0.25538759935978703638e-4_f64 * t45811 + 0.25538759935978703638e-4_f64 * t45813 - 0.11974241701863808564e0_f64 * t884 * t2024 * t30311 - t39827 + 0.85129199786595678796e-5_f64 * t45818 + 0.76616279807936110914e-4_f64 * t45822;
    t45824
}
