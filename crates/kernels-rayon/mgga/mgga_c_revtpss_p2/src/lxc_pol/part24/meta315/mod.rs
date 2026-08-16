//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta315(t22351: f64, t869: f64, t689: f64, t22005: f64, t4003: f64, t5744: f64, t2782: f64, t21981: f64, t4086: f64, t543: f64, t22009: f64, t6888: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22352, t22353, t22361, t22362, t22365, t22366, t22369, t22370, t22373, t22374, t22379) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1102(t22351, t869, t689, t22005, t4003, t5744, t2782, t21981, t4086, t543, t22009, t6888, t72);
    (t22352, t22353, t22361, t22362, t22365, t22366, t22369, t22370, t22373, t22374, t22379)
}
