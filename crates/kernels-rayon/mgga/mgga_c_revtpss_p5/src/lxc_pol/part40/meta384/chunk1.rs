//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1383/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1383(t16807: f64, t422: f64, t12552: f64, t1756: f64, t12555: f64, t3497: f64, t1196: f64, t16708: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12367: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64) {
    let t16809 = 0.621814e-1_f64 * t16807 * t422;
    let t16810 = t12552 * t1756;
    let t16811 = t12555 * t3497;
    let t16812 = t16810 * t16811;
    let t16814 = 0.10254018858216406658e4_f64 * t1196 * t16812;
    let t16820 = 0.41203703703703703704e-2_f64 * t16708;
    let t16821 = 0.12361111111111111111e-1_f64 * t16710;
    let t16822 = 0.61805555555555555556e-2_f64 * t16712;
    let t16831 = -t12367 + 0.82407407407407407407e-2_f64 * t12297 + 0.20601851851851851852e-2_f64 * t12299 - 0.61805555555555555556e-2_f64 * t12301 - 0.30902777777777777778e-2_f64 * t12303 + 0.41203703703703703704e-2_f64 * t16706 + t16820 - t16821 - t16822 + 0.10300925925925925926e-1_f64 * t16717 - 0.37083333333333333333e-1_f64 * t16722 - 0.12361111111111111111e-1_f64 * t16727 - 0.61805555555555555555e-2_f64 * t16731 + 0.55625000000000000001e-1_f64 * t16735 + 0.37083333333333333334e-1_f64 * t16740 + 0.18541666666666666667e-1_f64 * t16744 + 0.92708333333333333333e-2_f64 * t16748;
    (t16809, t16814, t16831)
}
