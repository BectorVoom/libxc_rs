//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2432;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta690(t39454: f64, t521: f64, t1333: f64, t9413: f64, t30: f64, t513: f64, t9603: f64, t33: f64, t516: f64, t9615: f64, t39552: f64, t562: f64, t560: f64, t9655: f64, t225: f64, t3896: f64, t39515: f64, t3900: f64, t9292: f64, t1419: f64, t9646: f64, t9648: f64, t1362: f64, t1363: f64, t39497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46291, t46297, t46310, t46328, t46359) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2432(t39454, t521, t1333, t9413, t30, t513, t9603, t33, t516, t9615, t39552, t562);
        let (t46362, t46368, t46369, t46378, t46385) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2433(t560, t9655, t225, t3896, t39515, t3900, t9292, t1419, t9646, t9648, t1362, t1363, t39497);
    (t46291, t46297, t46310, t46328, t46359, t46362, t46368, t46369, t46378, t46385)
}
