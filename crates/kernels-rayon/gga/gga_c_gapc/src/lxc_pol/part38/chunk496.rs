//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 496/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk496(t1001: f64, t2903: f64, t424: f64, t996: f64, t515: f64, t632: f64, t458: f64, t493: f64, t437: f64, t998: f64, t1031: f64, t22: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2904 = t2903 * t1001;
    let t2906 = t996 * t424;
    let t2907 = t2906 * t1001;
    let t2910 = t632 * t515;
    let t2911 = t996 * t2910;
    let t2912 = t493 * t458;
    let t2913 = t2911 * t2912;
    let t2915 = t998 * t437;
    let t2916 = t2903 * t2915;
    let t2919 = 1.0_f64 / t22 / t1031;
    (t2904, t2906, t2907, t2910, t2911, t2912, t2913, t2915, t2916, t2919)
}
