//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk720;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta138(t5618: f64, t807: f64, t1868: f64, t221: f64, t3979: f64, t3978: f64, t1885: f64, t3930: f64, t1856: f64, t72: f64, t757: f64, t539: f64, t73: f64, t1412: f64, t1883: f64, t4019: f64, t4018: f64, t241: f64, t4000: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5619, t5622, t5623, t5625, t5635, t5636, t5650) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk720(t5618, t807, t1868, t221, t3979, t3978, t1885, t3930, t1856, t72, t757, t539, t73);
        let (t5651, t5665, t5666, t5671) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk721(t1412, t1868, t1883, t221, t4019, t4018, t241, t4000, t820);
    (t5619, t5622, t5623, t5625, t5635, t5636, t5650, t5651, t5665, t5666, t5671)
}
