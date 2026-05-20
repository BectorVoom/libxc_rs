//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2530;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta705<F: Float>(t1419: F, t4056: F, t1429: F, t39501: F, t1398: F, t9840: F, t2482: F, t4114: F, t686: F, t72: F, t543: F, t1437: F, t10014: F, t10136: F, t215: F, t3923: F, t268: F, t4101: F, t10023: F, t4003: F, t10119: F, t5744: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46407, t46412, t46422, t46424, t46432, t46433, t46435) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2530::<F>(t1419, t4056, t1429, t39501, t1398, t9840, t2482, t4114, t686, t72, t543, t1437);
        let (t46443, t46448, t46452, t46454, t46456) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2531::<F>(t10014, t10136, t215, t3923, t268, t4101, t543, t10023, t4003, t10119, t1419, t5744);
    (t46407, t46412, t46422, t46424, t46432, t46433, t46435, t46443, t46448, t46452, t46454, t46456)
}
