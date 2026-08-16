//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 571/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk571<F: Float>(t4351: F, t4352: F, t1523: F, t418: F, t1407: F, t34: F, t39: F) -> (F, F, F, F) {
    let t4353 = t4351 * t4352;
    let t4355 = t1523 * t418;
    let t4356 = t4355 * t1407;
    let t4358 = t34 * t39;
    (t4353, t4355, t4356, t4358)
}
