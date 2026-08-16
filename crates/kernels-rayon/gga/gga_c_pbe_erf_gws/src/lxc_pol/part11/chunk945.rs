//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 945/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk945(t2052: f64, t4836: f64, t4839: f64, t4842: f64, t4845: f64, t4848: f64, t4853: f64, t4855: f64, t4857: f64, t4860: f64, t4863: f64, t1438: f64, t2515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21884 = t2052 * t2052;
    let t21885 = 1.0_f64 / t21884;
    let t21910 = 0.14035736153892489771e2_f64 * t4836;
    let t21911 = 0.86748647062252193714e-1_f64 * t4839;
    let t21912 = 0.13012297059337829057e0_f64 * t4842;
    let t21913 = 0.1926377843805564792e1_f64 * t4845;
    let t21914 = 0.65061485296689145286e-1_f64 * t4848;
    let t21917 = 384.0_f64 * t4853;
    let t21920 = 96.0_f64 * t4855;
    let t21921 = 576.0_f64 * t4857;
    let t21922 = 960.0_f64 * t4860;
    let t21923 = 480.0_f64 * t4863;
    let t21975 = t1438 * t2515;
    (t21885, t21910, t21911, t21912, t21913, t21914, t21917, t21920, t21921, t21922, t21923, t21975)
}
