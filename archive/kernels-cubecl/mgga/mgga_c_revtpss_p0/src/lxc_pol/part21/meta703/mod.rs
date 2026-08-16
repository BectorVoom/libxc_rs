//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2526;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta703<F: Float>(t39454: F, t521: F, t1333: F, t9413: F, t30: F, t513: F, t9603: F, t33: F, t516: F, t9615: F, t10008: F, t213: F, t10153: F, t2435: F, t2439: F, t3895: F, t4078: F, t39552: F, t562: F, t560: F, t9655: F, t225: F, t3896: F, t39515: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46291, t46297, t46310, t46328, t46350) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2526::<F>(t39454, t521, t1333, t9413, t30, t513, t9603, t33, t516, t9615, t10008, t213);
        let (t46353, t46356, t46359, t46362, t46368) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2527::<F>(t10153, t2435, t2439, t3895, t4078, t39552, t562, t560, t9655, t225, t3896, t39515);
    (t46291, t46297, t46310, t46328, t46350, t46353, t46356, t46359, t46362, t46368)
}
