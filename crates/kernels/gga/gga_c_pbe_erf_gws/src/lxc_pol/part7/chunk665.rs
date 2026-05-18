//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 665/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk665<F: Float>(t5299: F, t562: F, t1885: F, t1820: F, t1679: F, t586: F) -> (F, F, F, F) {
    let t5300 = t5299 * t562;
    let t5301 = t1885 * t5300;
    let t5303 = F::new(8.0) / F::new(5.0) * t1820 * t5301;
    let t5304 = t1679 * t586;
    (t5300, t5301, t5303, t5304)
}
