//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2524/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2524<F: Float>(t46089: F, t10414: F, t116: F, t112: F, t10199: F, t666: F, t2289: F, t2341: F, t2367: F, t10210: F, t625: F, t10214: F) -> (F, F, F, F, F, F, F, F) {
    let t46090 = F::new(20944.0) / F::new(81.0) * t46089;
    let t46126 = t10414 * t116;
    let t46143 = F::new(2618.0) / F::new(81.0) * t46089 * t112;
    let t46144 = t10199 * t666;
    let t46146 = t2289 * t2341;
    let t46148 = t2289 * t2367;
    let t46150 = t625 * t10210;
    let t46152 = t625 * t10214;
    (t46090, t46126, t46143, t46144, t46146, t46148, t46150, t46152)
}
