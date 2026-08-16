//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1524;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta287(t2240: f64, t602: f64, t2246: f64, t599: f64, t88: f64, t89: f64, t90: f64, t29: f64, t2248: f64, t644: f64, t2315: f64, t606: f64, t70: f64, t72: f64, t30: f64, t33: f64, t1927: f64, t2258: f64, t2251: f64, t627: f64, t9344: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10298, t10301, t10308, t10309, t10310, t10313, t10317) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1524(t2240, t602, t2246, t599, t88, t89, t90, t29, t2248, t644, t2315, t606, t70, t72);
        let (t10318, t10321, t10326) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1525(t30, t33, t1927, t2258, t2251, t627, t9344, zeta_threshold);
    (t10298, t10301, t10308, t10309, t10310, t10313, t10317, t10318, t10321, t10326)
}
