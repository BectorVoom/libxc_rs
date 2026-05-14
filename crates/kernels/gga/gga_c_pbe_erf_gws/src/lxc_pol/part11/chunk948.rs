//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 948/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk948<F: Float>(t13641: F, t2246: F, t13229: F, t4414: F, t12198: F, t2503: F, t13677: F, t376: F, t829: F, t830: F, t1114: F, t13140: F, t2365: F, t833: F, t1161: F, t353: F, t35541: F, t8599: F) -> (F, F, F, F, F, F) {
    let t46723 = t2246 * t13641;
    let t46731 = t4414 * t13229;
    let t46759 = t12198 * t2503;
    let t46763 = t829 * t830 * t13677 * t376;
    let t46858 = t1114 * t13140 * t2365 * t833;
    let t46862 = t8599 * t353 * t35541 * t1161;
    (t46723, t46731, t46759, t46763, t46858, t46862)
}
