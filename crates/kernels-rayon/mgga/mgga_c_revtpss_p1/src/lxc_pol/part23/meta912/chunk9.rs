//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2941/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2941(t77998: f64, t78010: f64, t78023: f64, t78035: f64, t78049: f64, t78061: f64, t78075: f64, t78088: f64, t915: f64, t935: f64, t23550: f64, t41583: f64) -> (f64, f64) {
    let t78094 = 1.0_f64 * t915 * (t77998 + t78010 + t78023 + t78035 + t78049 + t78061 + t78075 + t78088) * t935;
    let t78096 = 0.51726012919273400301e3_f64 * t41583 * t23550;
    (t78094, t78096)
}
