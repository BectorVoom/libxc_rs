//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1405;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1406;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta445(t22: f64, t46389: f64, t543: f64, t5735: f64, t1432: f64, t5763: f64, t9288: f64, t14202: f64, t9303: f64, t14238: f64, t2453: f64, t10139: f64, t14219: f64, t9285: f64, t1892: f64, t5744: f64, t786: f64, t1320: f64, t13632: f64, t1317: f64, t3857: f64, t5569: f64, t1856: f64, t512: f64, t9544: f64, t5571: f64, t9387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47967, t47971, t48005, t48007, t48036) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1405(t22, t46389, t543, t5735, t1432, t5763, t9288, t14202, t9303, t14238, t2453, t10139, t14219, t9285);
        let (t48084, t48152, t48225, t48227, t48243, t48262) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1406(t1892, t5744, t786, t1320, t13632, t1317, t3857, t5569, t1856, t512, t9544, t5571, t9387);
    (t47967, t47971, t48005, t48007, t48036, t48084, t48152, t48225, t48227, t48243, t48262)
}
