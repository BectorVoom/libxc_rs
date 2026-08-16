//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1910;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta565(t1873: f64, t94519: f64, t26004: f64, t5690: f64, t13951: f64, t2018: f64, t807: f64, t25240: f64, t3964: f64, t5617: f64, t543: f64, t97870: f64, t786: f64, t97961: f64, t1444: f64, t5675: f64, t25898: f64, t98040: f64, t1907: f64, t3889: f64, t25081: f64, t7897: f64, t1518: f64, t2319: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98260, t98269, t98281, t98285, t98299) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1910(t1873, t94519, t26004, t5690, t13951, t2018, t807, t25240, t3964, t5617, t543, t97870);
        let (t98308, t98362, t98380, t98436, t98450, t98484) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1911(t786, t97961, t1444, t5675, t25898, t98040, t1907, t3889, t25081, t7897, t1518, t2319);
    (t98260, t98269, t98281, t98285, t98299, t98308, t98362, t98380, t98436, t98450, t98484)
}
