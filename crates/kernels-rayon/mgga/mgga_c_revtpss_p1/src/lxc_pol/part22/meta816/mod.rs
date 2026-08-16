//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta816 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2925;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta816(t4146: f64, t1455: f64, t5808: f64, t1892: f64, t9646: f64, t9648: f64, t1904: f64, t47567: f64, t14110: f64, t47530: f64, t1427: f64, t1903: f64, t22: f64, t9647: f64, t2453: f64, t3908: f64, t5711: f64, t14296: f64, t9303: f64, t13738: f64, t686: f64, t72: f64, t9680: f64, t213: f64, t556: f64, t9656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47672, t47730, t47764, t47772, t47777, t47781) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2925(t4146, t1455, t5808, t1892, t9646, t9648, t1904, t47567, t14110, t47530, t1427, t1903, t22, t9647);
        let (t47784, t47786, t47791, t47793, t47794) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2926(t2453, t3908, t5711, t14296, t9303, t13738, t686, t72, t9680, t213, t556, t1903, t9656);
    (t47672, t47730, t47764, t47772, t47777, t47781, t47784, t47786, t47791, t47793, t47794)
}
