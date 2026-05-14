//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 788/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk788<F: Float>(t1478: F, t1480: F, t4579: F, t4585: F, t6054: F, t6056: F, t1378: F, t147: F, t2331: F, t1533: F, t510: F, t5651: F, t1590: F, t2030: F, t2032: F, t1592: F, t475: F) -> (F, F, F, F, F, F, F) {
    let t16415 = 0.10931146159029059066e-3 * t1478 * t4579 * t1480;
    let t16418 = 0.18276876377896586758e-4 * t6054 * t4585 * t6056;
    let t16422 = 0.67015213385620818113e-4 * t2331 * t147 * t1378 * t6056;
    let t16423 = t1533 * t1533;
    let t16428 = t5651 * t510 * t1533;
    let t16431 = t2030 * t1590;
    let t16432 = t16431 * t2032;
    let t16436 = t475 * t1592 * t2030;
    (t16415, t16418, t16422, t16423, t16428, t16432, t16436)
}
