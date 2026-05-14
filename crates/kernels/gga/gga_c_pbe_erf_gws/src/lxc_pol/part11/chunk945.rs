//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 945/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk945<F: Float>(t13500: F, t2323: F, t13337: F, t2164: F, t13171: F, t2306: F, t339: F, t13490: F, t3116: F, t6183: F, t11794: F, t8824: F, t13463: F, t13405: F, t8967: F, t13518: F, t2142: F) -> (F, F, F, F, F, F, F, F) {
    let t46524 = t2323 * t13500;
    let t46536 = t2164 * t13337;
    let t46544 = t2306 * t13171 * t339;
    let t46549 = t3116 * t6183 * t13490;
    let t46566 = t11794 * t8824;
    let t46596 = t2164 * t13463;
    let t46598 = t8967 * t13405;
    let t46615 = t13518 * t2142;
    (t46524, t46536, t46544, t46549, t46566, t46596, t46598, t46615)
}
