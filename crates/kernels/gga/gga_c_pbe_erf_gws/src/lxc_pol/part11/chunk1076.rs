//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1076/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1076<F: Float>(t17852: F, t3421: F, t3454: F, t587: F, t1017: F, t12464: F, t5543: F, t22811: F, t22813: F, t34395: F, t47315: F, t47319: F, t47323: F, t47325: F, t47327: F, t47331: F) -> (F, F, F) {
    let t47335 = F::new(16.0) / F::new(9.0) * t587 * t17852 * t3421 * t3454;
    let t47339 = F::new(32.0) / F::new(9.0) * t587 * t5543 * t12464 * t1017;
    let t47340 = F::new(4.0) / F::new(3.0) * t22811 + F::cast_from(0.72933333333333333331e0_f64) * t22813 + F::cast_from(0.19947266666666666666e0_f64) * t34395 + t47315 - t47319 - t47323 - t47325 + t47327 + t47331 + t47335 + t47339;
    (t47335, t47339, t47340)
}
