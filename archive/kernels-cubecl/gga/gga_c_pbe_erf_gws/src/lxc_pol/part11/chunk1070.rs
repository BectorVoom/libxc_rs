//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1070/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1070<F: Float>(t12111: F, t3083: F, t2501: F, t3717: F, t2370: F, t830: F, t3052: F, t9955: F, t45235: F, t6801: F, t12213: F, t3721: F) -> (F, F, F, F, F) {
    let t46914 = t3083 * t12111;
    let t46923 = t2501 * t3717;
    let t46925 = t2370 * t830 * t46923;
    let t46928 = t9955 * t3052;
    let t46930 = t45235 * t6801;
    let t46974 = t12213 * t3721;
    (t46914, t46925, t46928, t46930, t46974)
}
