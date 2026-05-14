//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 476/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk476<F: Float>(t1001: F, t2903: F, t424: F, t996: F, t515: F, t632: F, t458: F, t493: F, t437: F, t998: F, t1031: F, t22: F, t5: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2904 = t2903 * t1001;
    let t2906 = t996 * t424;
    let t2907 = t2906 * t1001;
    let t2910 = t632 * t515;
    let t2911 = t996 * t2910;
    let t2912 = t493 * t458;
    let t2913 = t2911 * t2912;
    let t2915 = t998 * t437;
    let t2916 = t2903 * t2915;
    let t2919 = 1.0 / t22 / t1031;
    let t2920 = t5 * t2919;
    (t2904, t2906, t2907, t2910, t2911, t2912, t2913, t2915, t2916, t2920)
}
