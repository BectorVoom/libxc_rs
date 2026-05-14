//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 974/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk974<F: Float>(t33105: F, t3414: F, t7062: F, t40604: F, t31102: F, t40655: F, t31200: F, t1827: F, t41514: F, t587: F, t950: F, t40672: F, t31225: F, t12647: F, t2612: F, t1017: F, t1820: F, t1885: F, t40676: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t47675 = 32.0 / 15.0 * t7062 * t33105 * t3414;
    let t47676 = 128.0 / 45.0 * t40604;
    let t47677 = 32.0 / 135.0 * t31102;
    let t47678 = 64.0 / 45.0 * t40655;
    let t47679 = 8.0 / 45.0 * t31200;
    let t47683 = 16.0 / 45.0 * t587 * t1827 * t41514 * t950;
    let t47684 = 32.0 / 15.0 * t40672;
    let t47685 = 32.0 / 135.0 * t31225;
    let t47687 = 16.0 / 5.0 * t2612 * t12647;
    let t47691 = 16.0 / 15.0 * t1820 * t1885 * t40676 * t1017;
    (t47675, t47676, t47677, t47678, t47679, t47683, t47684, t47685, t47687, t47691)
}
