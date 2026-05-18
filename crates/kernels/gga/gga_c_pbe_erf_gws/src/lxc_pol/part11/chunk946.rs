//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 946/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk946<F: Float>(t1333: F, t2515: F, t4847: F, t6967: F, t4844: F, t4838: F, t2840: F, t4805: F, t1114: F, t19776: F, t409: F, t7996: F) -> (F, F, F, F, F, F, F) {
    let t22063 = t1333 * t2515;
    let t22066 = t6967 * t4847;
    let t22068 = t6967 * t4844;
    let t22070 = t6967 * t4838;
    let t22084 = t2840 * t4805;
    let t22493 = t1114 * t19776;
    let t22590 = t409 * t7996;
    (t22063, t22066, t22068, t22070, t22084, t22493, t22590)
}
