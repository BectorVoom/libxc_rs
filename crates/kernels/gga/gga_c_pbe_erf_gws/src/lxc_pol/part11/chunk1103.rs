//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1103/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1103<F: Float>(t33105: F, t3414: F, t7062: F, t40604: F, t31102: F, t40655: F, t31200: F, t1827: F, t41514: F, t587: F, t950: F, t40672: F) -> (F, F, F, F, F, F, F) {
    let t47675 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t7062 * t33105 * t3414;
    let t47676 = F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t40604;
    let t47677 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t31102;
    let t47678 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t40655;
    let t47679 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t31200;
    let t47683 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t587 * t1827 * t41514 * t950;
    let t47684 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t40672;
    (t47675, t47676, t47677, t47678, t47679, t47683, t47684)
}
