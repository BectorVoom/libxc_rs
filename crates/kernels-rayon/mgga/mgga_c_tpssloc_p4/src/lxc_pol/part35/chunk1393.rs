//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1393/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1393(t20201: f64, t72: f64, t79: f64, t1433: f64, t5445: f64, t20288: f64, t5398: f64, t20218: f64, t605: f64, t1410: f64, t19299: f64, t28025: f64, t4028: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106836 = t72 * t79 * t20201;
    let t106842 = t72 * t1433 * t5445;
    let t106849 = t72 * t79 * t20288;
    let t106853 = t72 * t79 * t5398;
    let t106855 = t605 * t20218;
    let t106862 = t19299 * t1410;
    let t106889 = 6.0_f64 * t4028 * t28025;
    (t106836, t106842, t106849, t106853, t106855, t106862, t106889)
}
