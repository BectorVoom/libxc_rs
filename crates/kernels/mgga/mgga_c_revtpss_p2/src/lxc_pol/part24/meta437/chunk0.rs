//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1391/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1391<F: Float>(t10111: F, t1386: F, t9720: F, t281: F, t39644: F, t40650: F, t547: F, t550: F, t40688: F, t2682: F, t820: F, t2735: F, t5744: F) -> (F, F, F, F, F) {
    let t46856 = t10111 * t1386 * t9720;
    let t46885 = F::cast_from(0.47607864835161149081e-7_f64) * t39644 * t547 * t40650 * t550 * t281;
    let t46888 = t40688 * t547;
    let t46917 = t820 * t1386 * t2682;
    let t46929 = t2735 * t5744;
    (t46856, t46885, t46888, t46917, t46929)
}
