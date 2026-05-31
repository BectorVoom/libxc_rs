//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 852/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk852<F: Float>(t553: F, t6047: F, t1996: F, t5917: F, t1472: F, t2003: F, t671: F, t1750: F, t1778: F, t220: F, t7776: F, t211: F) -> (F, F, F, F, F) {
    let t16480 = F::cast_from(0.12408369628826103546e0_f64) * t6047 * t553;
    let t16481 = t1996 * t5917;
    let t16485 = F::cast_from(0.19878653761973934499e-1_f64) * t2003 * t1472 * t671;
    let t16486 = t1750 * t1778;
    let t16487 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16486;
    let t16488 = t7776 * t220;
    let t16490 = F::cast_from(112.0_f64) / F::cast_from(1215.0_f64) * t211 * t16488;
    (t16480, t16481, t16485, t16487, t16490)
}
