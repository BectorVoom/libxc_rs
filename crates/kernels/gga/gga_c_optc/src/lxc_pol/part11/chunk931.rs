//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 931/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk931<F: Float>(t22120: F, t587: F, t601: F, t6405: F, t2204: F, t2229: F, t1846: F, t1863: F, t6427: F, t2040: F, t8: F, t108: F, t117: F, t56: F, t6: F, t9771: F) -> (F, F, F, F, F, F, F) {
    let t22124 = 0.1403573615389248977e2 * t601 * t6405 * t22120 * t587;
    let t22126 = 70.0 / 3.0 * t2229 * t2204;
    let t22148 = 1.0 / t1863 / t1846;
    let t22152 = 0.12304676425209353917e5 * t601 * t22148 * t22120 * t6427;
    let t22154 = 1.0 / t8 / t2040;
    let t22158 = 455.0 / 243.0 * t108 * t22154 * t56 * t117;
    let t22166 = t9771 * t6;
    (t22124, t22126, t22148, t22152, t22154, t22158, t22166)
}
