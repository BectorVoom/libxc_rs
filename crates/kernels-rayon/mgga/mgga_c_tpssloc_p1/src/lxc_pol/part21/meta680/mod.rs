//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2489;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta680(t4119: f64, t828: f64, t46528: f64, t842: f64, t4261: f64, t9601: f64, t1516: f64, t40965: f64, t13347: f64, t2697: f64, t13210: f64, t9638: f64, t120: f64, t13170: f64, t13231: f64, t13258: f64, t41107: f64, t4250: f64, t13244: f64, t242: f64, t812: f64, t841: f64, t1484: f64, t2678: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46565, t46570, t46573, t46577, t46587, t46595) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2489(t4119, t828, t46528, t842, t4261, t9601, t1516, t40965, t13347, t2697, t13210, t9638);
        let (t46597, t46611, t46616, t46618, t46628, t46644) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2490(t120, t13170, t13231, t13258, t41107, t4250, t13244, t242, t812, t841, t1484, t2678);
    (t46565, t46570, t46573, t46577, t46587, t46595, t46597, t46611, t46616, t46618, t46628, t46644)
}
