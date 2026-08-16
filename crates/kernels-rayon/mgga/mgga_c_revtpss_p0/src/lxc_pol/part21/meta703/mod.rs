//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2526;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta703(t39454: f64, t521: f64, t1333: f64, t9413: f64, t30: f64, t513: f64, t9603: f64, t33: f64, t516: f64, t9615: f64, t10008: f64, t213: f64, t10153: f64, t2435: f64, t2439: f64, t3895: f64, t4078: f64, t39552: f64, t562: f64, t560: f64, t9655: f64, t225: f64, t3896: f64, t39515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46291, t46297, t46310, t46328, t46350) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2526(t39454, t521, t1333, t9413, t30, t513, t9603, t33, t516, t9615, t10008, t213);
        let (t46353, t46356, t46359, t46362, t46368) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2527(t10153, t2435, t2439, t3895, t4078, t39552, t562, t560, t9655, t225, t3896, t39515);
    (t46291, t46297, t46310, t46328, t46350, t46353, t46356, t46359, t46362, t46368)
}
