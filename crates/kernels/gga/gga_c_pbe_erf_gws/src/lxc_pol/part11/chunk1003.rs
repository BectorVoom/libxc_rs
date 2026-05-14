//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1003/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1003<F: Float>(t48112: F, t48113: F, t48114: F, t48115: F, t48117: F, t48119: F, t48120: F, t48122: F, t48124: F, t48127: F, t48128: F, t41524: F, t41562: F, t41570: F, t41573: F, t12651: F, t2615: F) -> (F, F, F, F, F, F) {
    let t48129 = t48112 - t48113 - t48114 - t48115 + t48117 + t48119 + t48120 + t48122 + t48124 + t48127 - t48128;
    let t48130 = 64.0 / 45.0 * t41524;
    let t48132 = 32.0 / 45.0 * t41562;
    let t48133 = 32.0 / 135.0 * t41570;
    let t48134 = 256.0 / 243.0 * t41573;
    let t48136 = 16.0 / 5.0 * t2615 * t12651;
    (t48129, t48130, t48132, t48133, t48134, t48136)
}
