//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 853/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk853(t1524: f64, t157: f64, t406: f64, t1444: f64, t372: f64, t1449: f64, t322: f64, t1175: f64, t1410: f64, t1460: f64, t513: f64, t930: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20972 = t1524 * t406 * t157;
    let t20987 = t1444 * t372;
    let t20992 = t1449 * t322;
    let t21099 = t1175 * t1410;
    let t21118 = t1460 * t322;
    let t21128 = t930 * t513;
    (t20972, t20987, t20992, t21099, t21118, t21128)
}
