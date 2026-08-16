//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1702;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta445(t10008: f64, t213: f64, t10153: f64, t2435: f64, t2439: f64, t3895: f64, t4078: f64, t39552: f64, t562: f64, t560: f64, t9655: f64, t225: f64, t4077: f64, t3896: f64, t39515: f64, t3900: f64, t9292: f64, t1419: f64, t9646: f64, t9648: f64, t10147: f64, t1357: f64, t689: f64, t1362: f64, t1363: f64, t39497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46350, t46353, t46356, t46359, t46362) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1702(t10008, t213, t10153, t2435, t2439, t3895, t4078, t39552, t562, t560, t9655, t225);
        let (t46363, t46368, t46369, t46378, t46381, t46385) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1703(t4077, t3896, t39515, t3900, t9292, t1419, t9646, t9648, t10147, t1357, t689, t1362, t1363, t39497);
    (t46350, t46353, t46356, t46359, t46362, t46363, t46368, t46369, t46378, t46381, t46385)
}
