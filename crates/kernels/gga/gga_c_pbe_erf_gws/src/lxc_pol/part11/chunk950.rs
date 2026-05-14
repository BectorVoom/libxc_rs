//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 950/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk950<F: Float>(t3052: F, t9955: F, t45235: F, t6801: F, t12213: F, t3721: F, t829: F, t830: F, t13615: F, t840: F, t12198: F, t3772: F, t898: F, t13173: F, t2358: F, t3733: F, t39470: F) -> (F, F, F, F, F, F, F, F) {
    let t46928 = t9955 * t3052;
    let t46930 = t45235 * t6801;
    let t46974 = t12213 * t3721;
    let t46976 = t829 * t830 * t46974;
    let t46996 = t840 * t13615;
    let t47008 = t12198 * t3052;
    let t47050 = t898 * t3772;
    let t47071 = t13173 * t2358;
    let t47082 = t39470 * t3733;
    (t46928, t46930, t46976, t46996, t47008, t47050, t47071, t47082)
}
