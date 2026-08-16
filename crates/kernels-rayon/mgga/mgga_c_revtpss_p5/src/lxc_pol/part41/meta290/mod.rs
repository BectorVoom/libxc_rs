//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1050;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1051;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta290(t785: f64, t860: f64, t780: f64, t2439: f64, t781: f64, t9292: f64, t861: f64, t867: f64, t786: f64, t2410: f64, t261: f64, t262: f64, t775: f64, t3335: f64, t389: f64, t1077: f64, t225: f64, t268: f64, t271: f64, t7021: f64, t2435: f64, t907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11030, t11040, t11044, t11064, t11088) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1050(t785, t860, t780, t2439, t781, t9292, t861, t867, t786, t2410, t261, t262, t775);
        let (t11108, t11121, t11132) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1051(t3335, t389, t1077, t225, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1052(t11132, t2435, t907);
    (t11030, t11040, t11044, t11064, t11088, t11108, t11121, t11132, t11133, t11134)
}
