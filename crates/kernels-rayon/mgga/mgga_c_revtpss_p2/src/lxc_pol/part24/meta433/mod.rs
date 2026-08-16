//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1383;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta433(t3853: f64, t3860: f64, t30: f64, t513: f64, t9603: f64, t33: f64, t516: f64, t9615: f64, t39552: f64, t562: f64, t560: f64, t9655: f64, t225: f64, t3896: f64, t39515: f64, t1362: f64, t1363: f64, t39497: f64, t1358: f64, t588: f64, t9647: f64, t4086: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46303, t46310, t46328, t46359, t46361) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1383(t3853, t3860, t30, t513, t9603, t33, t516, t9615, t39552, t562, t560, t9655);
        let (t46362, t46368, t46385, t46388, t46389) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1384(t225, t46361, t3896, t39515, t1362, t1363, t39497, t1358, t588, t9647, t4086, t9646);
    (t46303, t46310, t46328, t46359, t46362, t46368, t46385, t46388, t46389)
}
