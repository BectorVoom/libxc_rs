//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1331/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1331<F: Float>(t51201: F, t51215: F, t51222: F, t54019: F, t54021: F, t54024: F, t54027: F, t54029: F, t54031: F, t54033: F, t54035: F, t54039: F) -> F {
    let t55447 = -t54019 / F::cast_from(48.0_f64) - t54021 / F::cast_from(96.0_f64) - t54024 / F::cast_from(12.0_f64) + F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t51201 - t54027 / F::cast_from(12.0_f64) - t54029 / F::cast_from(12.0_f64) - t54031 / F::cast_from(96.0_f64) + F::cast_from(5.0_f64) / F::cast_from(48.0_f64) * t54033 - t54035 / F::cast_from(64.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t51215 + F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t51222 + t54039 / F::cast_from(24.0_f64);
    t55447
}
