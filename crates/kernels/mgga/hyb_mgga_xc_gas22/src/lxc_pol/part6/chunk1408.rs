//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1408/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1408<F: Float>(t22050: F, t22054: F, t22058: F, t22061: F, t22064: F, t22068: F, t22072: F, t22076: F, t22080: F, t22084: F, t22089: F, t22090: F, t22094: F, t22095: F, t26007: F, t26010: F, t26012: F) -> F {
    let t30451 = t22050 + t22054 + t22058 + F::cast_from(48.0_f64) * t22061 - F::cast_from(0.69263436422725855034e2_f64) * t26007 + F::cast_from(2.0_f64) * t22064 + t22068 - F::cast_from(240.0_f64) * t26010 - F::cast_from(0.11393789434848516923e-2_f64) * t26012 - t22072 + t22076 + F::cast_from(192.0_f64) * t22080 + F::cast_from(96.0_f64) * t22084 - t22089 - F::cast_from(0.70178683471615754484e1_f64) * t22090 + t22094 - F::cast_from(160.0_f64) * t22095;
    t30451
}
