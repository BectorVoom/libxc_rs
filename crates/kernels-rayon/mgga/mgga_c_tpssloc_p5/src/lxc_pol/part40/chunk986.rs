//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 986/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk986(t2989: f64, t3966: f64, t2960: f64, t4506: f64, t10224: f64, t1592: f64, t973: f64, t4528: f64, t1599: f64, t698: f64, t135: f64, t4542: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13861 = t2989 * t3966;
    let t13893 = 0.49382716049382716048e-3_f64 * t2960 * t4506;
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    let t13907 = 0.14814814814814814814e-2_f64 * t2960 * t4528;
    let t13908 = t698 * t1599;
    let t13909 = t973 * t13908;
    let t13913 = t135 * t4542;
    (t13861, t13893, t13896, t13907, t13909, t13913)
}
