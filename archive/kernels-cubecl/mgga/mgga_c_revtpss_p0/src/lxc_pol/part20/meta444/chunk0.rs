//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1699/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1699<F: Float>(t30: F, t3889: F, t3853: F, t3860: F, t10179: F, t4147: F, t513: F, t9603: F, t3834: F, t2257: F, t1344: F, t3874: F, t39456: F, t9344: F, t9605: F, t9608: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t46298 = t3889 * t3889;
    let t46302 = t3860 * t3853;
    let t46303 = F::cast_from(72.0_f64) * t46302;
    let t46304 = t10179 * t4147;
    let t46310 = F::cast_from(1.0_f64) / t513 / t9603 / t30;
    let t46311 = t3834 * t3834;
    let t46317 = t2257 * t2257;
    let t46325 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46310 * t46311 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t9605 * t3834 * t2257 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3874 * t46317 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9608 * t9344 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1344 * t39456);
    (t46298, t46303, t46304, t46311, t46317, t46325)
}
