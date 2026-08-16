//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1704;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta446(t1358: f64, t588: f64, t9647: f64, t4086: f64, t9646: f64, t1399: f64, t22: f64, t555: f64, t9890: f64, t10040: f64, t2435: f64, t10039: f64, t2439: f64, t2777: f64, t4003: f64, t1419: f64, t4056: f64, t1429: f64, t39501: f64, t9994: f64, t1398: f64, t9840: f64, t2482: f64, t4114: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46388, t46392, t46394, t46398, t46401) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1704(t1358, t588, t9647, t4086, t9646, t1399, t22, t555, t9890, t10040, t2435, t10039, t2439, t2777);
        let (t46403, t46407, t46412, t46416, t46422, t46424) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1705(t4003, t9890, t1419, t4056, t1429, t39501, t9994, t1398, t9840, t2482, t4114, t686, t72);
    (t46388, t46392, t46394, t46398, t46401, t46403, t46407, t46412, t46416, t46422, t46424)
}
