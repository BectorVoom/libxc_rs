//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1116/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1116<F: Float>(t41056: F, t41061: F, t41069: F, t41074: F, t12550: F, t2615: F, t47400: F, t587: F, t590: F, t591: F, t10848: F, t3531: F) -> (F, F, F, F, F, F, F) {
    let t47809 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t41056;
    let t47810 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t41061;
    let t47811 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t41069;
    let t47812 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t41074;
    let t47814 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2615 * t12550;
    let t47818 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t587 * t590 * t591 * t47400;
    let t47820 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t10848 * t3531;
    (t47809, t47810, t47811, t47812, t47814, t47818, t47820)
}
