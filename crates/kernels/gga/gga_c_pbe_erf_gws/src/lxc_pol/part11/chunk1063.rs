//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1063/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1063<F: Float>(t13507: F, t2323: F, t13590: F, t2289: F, t1114: F, t13140: F, t2132: F, t343: F, t44254: F, t2121: F, t337: F, t13500: F) -> (F, F, F, F, F) {
    let t46430 = t2323 * t13507;
    let t46436 = t2289 * t13590;
    let t46446 = t1114 * t13140 * t2132;
    let t46449 = t44254 * t343;
    let t46451 = t2121 * t337 * t46449;
    let t46524 = t2323 * t13500;
    (t46430, t46436, t46446, t46451, t46524)
}
