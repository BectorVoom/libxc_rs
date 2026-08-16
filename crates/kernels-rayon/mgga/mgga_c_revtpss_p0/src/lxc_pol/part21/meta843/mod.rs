//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta843 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta843(t16903: f64, t698: f64, t141: f64, t3417: f64, t56192: f64, t56196: f64, t56201: f64, t56205: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t58186, t58189, t58192, t58195, t58198, t58200) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3156(t16903, t698, t141, t3417, t56192, t56196, t56201, t56205, t43865, t43883, t43888, t43890, t43892, t43894, t43896);
    (t58186, t58189, t58192, t58195, t58198, t58200)
}
