//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 949/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk949<F: Float>(t353: F, t3886: F, t4386: F, t8713: F, t13121: F, t22493: F, t13684: F, t4414: F, t13619: F, t840: F, t12111: F, t3083: F, t2501: F, t3717: F, t2370: F, t830: F) -> (F, F, F, F, F, F) {
    let t46867 = t4386 * t353 * t8713 * t3886;
    let t46870 = t22493 * t13121;
    let t46872 = t4414 * t13684;
    let t46892 = t840 * t13619;
    let t46914 = t3083 * t12111;
    let t46923 = t2501 * t3717;
    let t46925 = t2370 * t830 * t46923;
    (t46867, t46870, t46872, t46892, t46914, t46925)
}
