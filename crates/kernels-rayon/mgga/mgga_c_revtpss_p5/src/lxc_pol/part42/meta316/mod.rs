//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta316(t9395: f64, t2626: f64, t5571: f64, t1856: f64, t2608: f64, t512: f64, t9408: f64, t9411: f64, t1317: f64, t5567: f64, t2496: f64, t9597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13623, t13630, t13633, t13634, t13635, t13643, t13652, t13664) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1089(t9395, t2626, t5571, t1856, t2608, t512, t9408, t9411, t1317, t5567, t2496, t9597);
    (t13623, t13630, t13633, t13634, t13635, t13643, t13652, t13664)
}
