//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 855/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk855<F: Float>(t4842: F, t4845: F, t4848: F, t4853: F, t4855: F, t4857: F, t4860: F, t4863: F, t1438: F, t2515: F, t1333: F, t4847: F, t6967: F, t4844: F, t4838: F, t2840: F, t4805: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21912 = 0.13012297059337829057e0 * t4842;
    let t21913 = 0.1926377843805564792e1 * t4845;
    let t21914 = 0.65061485296689145286e-1 * t4848;
    let t21917 = 384.0 * t4853;
    let t21920 = 96.0 * t4855;
    let t21921 = 576.0 * t4857;
    let t21922 = 960.0 * t4860;
    let t21923 = 480.0 * t4863;
    let t21975 = t1438 * t2515;
    let t22063 = t1333 * t2515;
    let t22066 = t6967 * t4847;
    let t22068 = t6967 * t4844;
    let t22070 = t6967 * t4838;
    let t22084 = t2840 * t4805;
    (t21912, t21913, t21914, t21917, t21920, t21921, t21922, t21923, t21975, t22063, t22066, t22068, t22070, t22084)
}
