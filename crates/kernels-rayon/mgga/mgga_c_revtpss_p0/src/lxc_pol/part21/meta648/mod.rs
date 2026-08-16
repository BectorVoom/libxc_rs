//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta648(t11199: f64, t988: f64, t378: f64, t11198: f64, t340: f64, t338: f64, t11119: f64, t384: f64, t225: f64, t41306: f64, t3057: f64, t3259: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t42051, t42052, t42059, t42060, t42067, t42078, t42107) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2433(t11199, t988, t378, t11198, t340, t338, t11119, t384, t225, t41306, t3057, t3259);
    (t42051, t42052, t42059, t42060, t42067, t42078, t42107)
}
