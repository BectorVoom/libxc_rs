//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 584/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk584<F: Float>(t41: F, t4562: F, t1602: F, t700: F, t1383: F, t536: F, t1477: F, t6: F) -> (F, F, F, F) {
    let t4563 = t41 * t4562;
    let t4566 = t1602 * t700;
    let t4568 = t536 * t1383;
    let t4573 = t6 * t1477;
    (t4563, t4566, t4568, t4573)
}
