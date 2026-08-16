//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta592(t1214: f64, t21471: f64, t5464: f64, t1770: f64, t5462: f64, t5477: f64, t4003: f64, t5658: f64, t1398: f64, t9994: f64, t1877: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t21472, t21483, t21500, t21579, t21990, t22016, t22229) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2309(t1214, t21471, t5464, t1770, t5462, t5477, t4003, t5658, t1398, t9994, t1877, t73);
    (t21472, t21483, t21500, t21579, t21990, t22016, t22229)
}
