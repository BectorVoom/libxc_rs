//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta879 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3048;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta879(t14593: f64, t2470: f64, t874: f64, t1558: f64, t2482: f64, t2801: f64, t2815: f64, t10547: f64, t14606: f64, t10538: f64, t14605: f64, t49180: f64, t14586: f64, t2645: f64, t10529: f64, t2782: f64, t10535: f64, t136: f64, t2457: f64, t4424: f64, t10523: f64, t14568: f64, t4423: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51587, t51598, t51600, t51603) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3048(t14593, t2470, t874, t1558, t2482, t2801, t2815, t10547, t14606, t10538, t14605, t49180);
        let (t51610, t51614, t51617, t51621) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3049(t14586, t2645, t10529, t2782, t10535, t136, t2457, t4424, t10523, t14568, t2482, t2801, t4423, t879);
    (t51587, t51598, t51600, t51603, t51610, t51614, t51617, t51621)
}
