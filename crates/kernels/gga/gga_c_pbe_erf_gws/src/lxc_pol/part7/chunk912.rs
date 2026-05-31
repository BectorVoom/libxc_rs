//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 912/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk912<F: Float>(t5102: F, t633: F, t1672: F, t1725: F, t211: F, t1406: F, t1820: F, t1885: F, t5299: F, t5292: F, t9: F, t5295: F, t587: F) -> (F, F, F, F) {
    let t17163 = t633 * t5102;
    let t17164 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17163;
    let t17166 = t211 * t1672 * t1725;
    let t17167 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t17166;
    let t17171 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t1820 * t1885 * t5299 * t1406;
    let t17172 = t9 * t5292;
    let t17174 = t587 * t17172 * t5295;
    (t17164, t17167, t17171, t17174)
}
