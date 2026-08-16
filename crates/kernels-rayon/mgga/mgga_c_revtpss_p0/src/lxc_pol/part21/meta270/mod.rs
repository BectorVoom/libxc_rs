//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta270(t4003: f64, t9768: f64, t9934: f64, t2661: f64, t532: f64, t549: f64, t240: f64, t72: f64, t828: f64, t9400: f64, t595: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9935, t9936, t9937, t9940, t9941, t9942, t9944, t9948) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1488(t4003, t9768, t9934, t2661, t532, t549, t240, t72, t828, t9400, t595, t66);
    (t9935, t9936, t9937, t9940, t9941, t9942, t9944, t9948)
}
