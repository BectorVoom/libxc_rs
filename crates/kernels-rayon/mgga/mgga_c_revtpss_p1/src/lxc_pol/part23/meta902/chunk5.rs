//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2884/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2884(t23421: f64, t892: f64, t18865: f64, t18871: f64, t18875: f64, t2403: f64, t77029: f64, t77032: f64, t77036: f64, t77038: f64, t77039: f64, t77040: f64, t77041: f64, t77045: f64, t775: f64) -> f64 {
    let t77460 = t23421 * t892;
    let t77467 = -9.0_f64 * t18865 * t18875 * t2403 + 18.0_f64 * t18871 * t18875 * t2403 + 3.0_f64 * t2403 * t77460 * t775 + t77029 + t77032 + t77036 + t77038 + t77039 + t77040 + t77041 + t77045;
    t77467
}
