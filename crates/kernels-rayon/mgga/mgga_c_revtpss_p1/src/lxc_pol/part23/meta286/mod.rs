//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta286(t2453: f64, t861: f64, t2458: f64, t785: f64, t860: f64, t780: f64, t2439: f64, t781: f64, t9292: f64, t867: f64, t786: f64, t2410: f64, t261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11018, t11019, t11028, t11029, t11030, t11040, t11043, t11044, t11064) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1511(t2453, t861, t2458, t785, t860, t780, t2439, t781, t9292, t867, t786, t2410, t261);
    (t11018, t11019, t11028, t11029, t11030, t11040, t11043, t11044, t11064)
}
