//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 802/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk802<F: Float>(t4882: F, t5211: F, t5213: F, t5210: F, t735: F, t5214: F, t1403: F) -> (F, F, F) {
    let t16665 = 64.0 / 15.0 * t5211 * t5213 * t4882;
    let t16666 = t5210 * t735;
    let t16667 = t16666 * t5214;
    let t16668 = 128.0 / 45.0 * t16667;
    let t16669 = t1403 * t1403;
    (t16665, t16668, t16669)
}
