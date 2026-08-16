//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1921;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta427(t1903: f64, t4131: f64, t4076: f64, t4077: f64, t9657: f64, t1444: f64, t5774: f64, t10171: f64, t13727: f64, t13733: f64, t13737: f64, t1424: f64, t1904: f64, t9632: f64, t9636: f64, t9639: f64, t9642: f64, t9650: f64, t13716: f64, t1414: f64, t828: f64, t221: f64, t3979: f64, t5591: f64, t3978: f64, t3989: f64, t5614: f64, t5622: f64, t9765: f64, t1408: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13738, t13739, t13743, t13746, t13747, t13750) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1921(t1903, t4131, t4076, t4077, t9657, t1444, t5774, t10171, t13727, t13733, t13737, t1424, t1904, t9632, t9636, t9639, t9642, t9650);
        let (t13756, t13760, t13762, t13763, t13765, t13767) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1922(t13716, t1414, t828, t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240);
    (t13738, t13739, t13743, t13746, t13747, t13750, t13756, t13760, t13762, t13763, t13765, t13767)
}
