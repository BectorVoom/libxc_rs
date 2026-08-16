//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1735/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1735<F: Float>(t1386: F, t2682: F, t820: F, t3940: F, t1399: F, t3960: F, t9816: F, t9818: F, t3829: F, t4003: F, t2735: F, t5744: F) -> (F, F, F, F) {
    let t46917 = t820 * t1386 * t2682;
    let t46918 = t46917 * t3940;
    let t46922 = t9816 * t9818 * t3960 * t1399;
    let t46924 = t4003 * t3829;
    let t46929 = t2735 * t5744;
    (t46918, t46922, t46924, t46929)
}
