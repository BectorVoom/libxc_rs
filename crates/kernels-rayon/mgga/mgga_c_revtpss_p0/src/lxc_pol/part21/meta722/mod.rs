//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta722 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta722(t190: f64, t22: f64, t519: f64, t39762: f64, t47065: f64, t1317: f64, t9545: f64, t1340: f64, t40129: f64, t72: f64, t757: f64, t9363: f64) -> (f64, f64, f64, f64, f64) {
        let (t47070, t47072, t47073, t47076, t47078) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2562(t190, t22, t519, t39762, t47065, t1317, t9545, t1340, t40129, t72, t757, t9363);
    (t47070, t47072, t47073, t47076, t47078)
}
