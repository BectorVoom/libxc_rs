//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 945/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk945<F: Float>(t2052: F, t4836: F, t4839: F, t4842: F, t4845: F, t4848: F, t4853: F, t4855: F, t4857: F, t4860: F, t4863: F, t1438: F, t2515: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21884 = t2052 * t2052;
    let t21885 = F::new(1.0) / t21884;
    let t21910 = F::cast_from(0.14035736153892489771e2_f64) * t4836;
    let t21911 = F::cast_from(0.86748647062252193714e-1_f64) * t4839;
    let t21912 = F::cast_from(0.13012297059337829057e0_f64) * t4842;
    let t21913 = F::cast_from(0.1926377843805564792e1_f64) * t4845;
    let t21914 = F::cast_from(0.65061485296689145286e-1_f64) * t4848;
    let t21917 = F::new(384.0) * t4853;
    let t21920 = F::new(96.0) * t4855;
    let t21921 = F::new(576.0) * t4857;
    let t21922 = F::new(960.0) * t4860;
    let t21923 = F::new(480.0) * t4863;
    let t21975 = t1438 * t2515;
    (t21885, t21910, t21911, t21912, t21913, t21914, t21917, t21920, t21921, t21922, t21923, t21975)
}
