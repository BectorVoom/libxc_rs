//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2881;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta790(t39552: f64, t562: f64, t560: f64, t9655: f64, t225: f64, t3896: f64, t39515: f64, t3900: f64, t9292: f64, t1419: f64, t9646: f64, t9648: f64, t1362: f64, t1363: f64, t39497: f64, t1358: f64, t588: f64, t9647: f64, t4086: f64, t1399: f64, t22: f64, t555: f64, t10040: f64, t2435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46359, t46362, t46368, t46369, t46378) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2881(t39552, t562, t560, t9655, t225, t3896, t39515, t3900, t9292, t1419, t9646, t9648);
        let (t46385, t46388, t46389, t46392, t46398) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2882(t1362, t1363, t39497, t1358, t588, t9647, t4086, t9646, t1399, t22, t555, t10040, t2435);
    (t46359, t46362, t46368, t46369, t46378, t46385, t46388, t46389, t46392, t46398)
}
