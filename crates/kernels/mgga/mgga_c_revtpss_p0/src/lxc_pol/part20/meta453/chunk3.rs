//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1732/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1732<F: Float>(t10001: F, t221: F, t4019: F, t9912: F, t10111: F, t1386: F, t9720: F, t1390: F, t1399: F, t685: F, t9970: F, t9976: F) -> (F, F, F) {
    let t46853 = t10001 * t4019 * t221 * t9912;
    let t46856 = t10111 * t1386 * t9720;
    let t46859 = t46856 * t1390 * t685 * t1399;
    let t46861 = t9976 * t9970;
    (t46853, t46859, t46861)
}
