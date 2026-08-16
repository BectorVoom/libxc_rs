//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2102;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta589(t46573: f64, t1516: f64, t40965: f64, t242: f64, t812: f64, t841: f64, t41115: f64, t4250: f64, t4166: f64, t9637: f64, t13176: f64, t2638: f64, t4179: f64, t820: f64, t836: f64, t9972: f64, t12985: f64, t9577: f64, t212: f64, t4119: f64, t2586: f64, t9523: f64, t4138: f64, t9541: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46574, t46577, t46628, t46650, t46657, t46667) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2102(t46573, t1516, t40965, t242, t812, t841, t41115, t4250, t4166, t9637, t13176, t2638);
        let (t46692, t46741, t46764, t46769, t46770) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2103(t4179, t820, t812, t836, t9972, t12985, t9577, t212, t4119, t2586, t9523, t4138, t9541);
    (t46574, t46577, t46628, t46650, t46657, t46667, t46692, t46741, t46764, t46769, t46770)
}
