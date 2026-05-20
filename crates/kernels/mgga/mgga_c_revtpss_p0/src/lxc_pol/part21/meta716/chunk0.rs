//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2552/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2552<F: Float>(t2735: F, t9792: F, t1413: F, t46826: F, t1376: F, t40769: F, t3989: F, t9986: F, t10001: F, t221: F, t4019: F, t9912: F) -> (F, F, F, F, F) {
    let t46835 = t2735 * t9792;
    let t46837 = t46835 * t1413 * t46826;
    let t46840 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t1376;
    let t46846 = t3989 * t9986;
    let t46853 = t10001 * t4019 * t221 * t9912;
    (t46835, t46837, t46840, t46846, t46853)
}
