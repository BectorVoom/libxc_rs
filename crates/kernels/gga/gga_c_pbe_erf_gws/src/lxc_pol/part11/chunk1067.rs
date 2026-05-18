//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1067/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1067<F: Float>(t46705: F, t6148: F, t830: F, t13212: F, t8662: F, t12198: F, t3047: F, t13105: F, t35014: F, t13656: F, t6832: F, t13641: F, t2246: F) -> (F, F, F, F, F, F) {
    let t46707 = t6148 * t830 * t46705;
    let t46710 = t8662 * t13212;
    let t46712 = t12198 * t3047;
    let t46714 = t35014 * t13105;
    let t46717 = t6832 * t13656;
    let t46723 = t2246 * t13641;
    (t46707, t46710, t46712, t46714, t46717, t46723)
}
