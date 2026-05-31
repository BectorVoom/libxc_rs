//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1240/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1240<F: Float>(t4836: F, t4839: F, t4842: F, t4845: F, t4848: F, t4853: F, t4855: F, t4857: F, t4860: F, t4863: F, t19476: F, t19492: F, t19494: F, t19498: F, t19513: F, t19526: F, t21889: F, t21905: F, t4851: F, t6080: F, t6860: F, t6863: F, t6866: F, t6870: F, t7: F) -> F {
    let t21910 = F::cast_from(0.14035736153892489771e2_f64) * t4836;
    let t21911 = F::cast_from(0.86748647062252193714e-1_f64) * t4839;
    let t21912 = F::cast_from(0.13012297059337829057e0_f64) * t4842;
    let t21913 = F::cast_from(0.1926377843805564792e1_f64) * t4845;
    let t21914 = F::cast_from(0.65061485296689145286e-1_f64) * t4848;
    let t21917 = F::cast_from(384.0_f64) * t4853;
    let t21920 = F::cast_from(96.0_f64) * t4855;
    let t21921 = F::cast_from(576.0_f64) * t4857;
    let t21922 = F::cast_from(960.0_f64) * t4860;
    let t21923 = F::cast_from(480.0_f64) * t4863;
    let t21924 = F::cast_from(36.0_f64) * t6860 + F::cast_from(72.0_f64) * t6863 + t7 * (t19476 + t19492 + t19494 + t19498 + t19513 + t19526 + t21889 + t21905) - t21910 - t21911 - t21912 + t21913 + t21914 + F::cast_from(0.13012297059337829058e0_f64) * t4851 + F::cast_from(24.0_f64) * t6866 - t21917 + F::cast_from(0.82152657680133333336e1_f64) * t6080 - F::cast_from(12.0_f64) * t6870 + t21920 - t21921 + t21922 - t21923;
    t21924
}
