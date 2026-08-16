//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2558/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2558<F: Float>(t1333: F, t9410: F, t1320: F, t9428: F, t1331: F, t9413: F, t3853: F, t3863: F, t9561: F, t9554: F, t1340: F, t40086: F) -> (F, F, F, F, F, F, F, F) {
    let t46971 = t9410 * t1333;
    let t46973 = t1320 * t9428;
    let t46975 = t9410 * t1331;
    let t46977 = t9413 * t1331;
    let t46979 = t3863 * t3853;
    let t46981 = t1320 * t9561;
    let t46983 = t1320 * t9554;
    let t46988 = F::cast_from(0.62337092780453269531e3_f64) * t1340 * t40086;
    (t46971, t46973, t46975, t46977, t46979, t46981, t46983, t46988)
}
