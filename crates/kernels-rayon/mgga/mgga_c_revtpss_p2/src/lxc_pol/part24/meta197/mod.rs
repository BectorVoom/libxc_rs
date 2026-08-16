//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta197(t10115: f64, t557: f64, t1429: f64, t9292: f64, t3964: f64, t4096: f64, t9285: f64, t2453: f64, t4100: f64, t562: f64, t64: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t10117, t10126, t10129, t10139, t10157, t10199) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk931(t10115, t557, t1429, t9292, t3964, t4096, t9285, t2453, t4100, t562, t64, t843);
    (t10117, t10126, t10129, t10139, t10157, t10199)
}
