//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1113/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1113<F: Float>(t24131: F, t1046: F, t12519: F, t3445: F, t3488: F, t17444: F, t47377: F, t5400: F, t639: F, t47766: F, t7115: F, t7505: F) -> (F, F, F, F, F) {
    let t47782 = F::cast_from(64.0_f64) / F::cast_from(405.0_f64) * t24131;
    let t47784 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t12519 * t1046;
    let t47786 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t3488 * t3445;
    let t47790 = F::cast_from(128.0_f64) / F::cast_from(27.0_f64) * t639 * t5400 * t17444 * t47377;
    let t47793 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t7115 * t7505 * t47766;
    (t47782, t47784, t47786, t47790, t47793)
}
