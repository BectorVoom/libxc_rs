//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1792;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta335(t11025: f64, t689: f64, t785: f64, t860: f64, t780: f64, t2439: f64, t2772: f64, t779: f64, t781: f64, t9292: f64, t861: f64, t867: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11026, t11028, t11029, t11030, t11036, t11037, t11040, t11043) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1792(t11025, t689, t785, t860, t780, t2439, t2772, t779, t781, t9292, t861, t867);
        let t11044 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1793(t11043, t786);
    (t11026, t11028, t11029, t11030, t11036, t11037, t11040, t11043, t11044)
}
