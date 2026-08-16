//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1330/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1330<F: Float>(t15018: F, t840: F, t53896: F, t54014: F, t53994: F, t53996: F, t53998: F, t54000: F, t54002: F, t54004: F, t54006: F, t54008: F, t54010: F, t54012: F, t54016: F) -> (F, F, F) {
    let t55420 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t15018;
    let t55421 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t53896;
    let t55432 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54014;
    let t55434 = t53994 / F::cast_from(16.0_f64) + t53996 / F::cast_from(12.0_f64) + t53998 / F::cast_from(12.0_f64) - t54000 / F::cast_from(96.0_f64) - t54002 / F::cast_from(192.0_f64) + t54004 / F::cast_from(12.0_f64) - t54006 / F::cast_from(24.0_f64) - t54008 / F::cast_from(48.0_f64) + t54010 / F::cast_from(8.0_f64) - t54012 / F::cast_from(24.0_f64) + t55432 + t54016 / F::cast_from(96.0_f64);
    (t55420, t55421, t55434)
}
