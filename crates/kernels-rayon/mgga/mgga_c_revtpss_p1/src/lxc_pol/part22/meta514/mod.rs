//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta514(t16807: f64, t422: f64, t12552: f64, t1756: f64, t12555: f64, t3497: f64, t1196: f64, t16708: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12367: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16809, t16811, t16812, t16814, t16820, t16821, t16822, t16831) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2279(t16807, t422, t12552, t1756, t12555, t3497, t1196, t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12367, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16809, t16811, t16812, t16814, t16820, t16821, t16822, t16831)
}
