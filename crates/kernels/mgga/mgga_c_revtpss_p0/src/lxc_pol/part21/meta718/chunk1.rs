//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2557/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2557<F: Float>(t4086: F, t9801: F, t9846: F, t9744: F, t9966: F, t3855: F, t3860: F, t1320: F, t9545: F, t3857: F, t40082: F, t512: F, t520: F) -> (F, F, F, F, F, F, F) {
    let t46946 = t9801 * t4086;
    let t46947 = t46946 * t9846;
    let t46949 = t9744 * t9966;
    let t46960 = t3860 * t3855;
    let t46963 = F::new(16.0) * t1320 * t9545;
    let t46967 = t3857 * t3855;
    let t46970 = t512 * t520 * t40082;
    (t46946, t46947, t46949, t46960, t46963, t46967, t46970)
}
