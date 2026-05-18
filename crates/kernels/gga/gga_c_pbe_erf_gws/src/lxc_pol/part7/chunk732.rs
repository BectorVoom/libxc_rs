//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 732/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk732<F: Float>(t5990: F, t5993: F, t5994: F, t5996: F, t5999: F, t6003: F, t6005: F, t6008: F, t6012: F, t6015: F, t6018: F, t6021: F) -> F {
    let t6023 = -F::new(0.18903244333884670701e0) * t5990 - t5993 + F::new(0.94516221669423353502e-1) * t5994 + F::new(0.18903244333884670701e0) * t5996 + t5999 + t6003 - t6005 + F::new(0.19753890328909480882e-1) * t6008 + t6012 + t6015 - F::new(0.59261670986728442646e-2) * t6018 - F::new(0.11852334197345688529e-1) * t6021;
    t6023
}
