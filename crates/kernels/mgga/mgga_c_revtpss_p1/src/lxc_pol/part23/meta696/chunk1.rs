//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2445/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2445<F: Float>(t46786: F, t46888: F, t1386: F, t2682: F, t820: F, t2735: F, t5744: F, t4086: F, t9801: F, t9846: F, t1320: F, t9545: F) -> (F, F, F, F, F, F) {
    let t46889 = t46888 * t46786;
    let t46917 = t820 * t1386 * t2682;
    let t46929 = t2735 * t5744;
    let t46946 = t9801 * t4086;
    let t46947 = t46946 * t9846;
    let t46963 = F::cast_from(16.0_f64) * t1320 * t9545;
    (t46889, t46917, t46929, t46946, t46947, t46963)
}
