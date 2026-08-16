//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 791/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk791<F: Float>(t2153: F, t837: F, t863: F, t2160: F, t2289: F, t2293: F, t6247: F, t904: F, t916: F, t2262: F, t344: F, t362: F) -> (F, F, F, F, F, F) {
    let t6542 = t863 * t2153 * t837;
    let t6543 = t6542 * t2160;
    let t6544 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t6543;
    let t6545 = t2289 * t2293;
    let t6548 = t916 * t904 * t6247;
    let t6552 = F::cast_from(1.0_f64) / t2262 / t344;
    let t6553 = t6552 * t362;
    (t6542, t6544, t6545, t6548, t6552, t6553)
}
