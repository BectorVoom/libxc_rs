//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1297/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1297(t15573: f64, t2173: f64, t28957: f64, t28951: f64, t1003: f64, t100575: f64, t100578: f64, t100580: f64, t100583: f64, t100586: f64, t100975: f64, t27772: f64, t28948: f64, t28952: f64, t7687: f64, t7696: f64, t7703: f64, t96019: f64) -> (f64, f64, f64) {
    let t101342 = t2173 * t15573 * t28957;
    let t101355 = t15573 * t28951;
    let t101356 = t2173 * t101355;
    let t101363 = -0.33163888888888888888e-2_f64 * t100575 - 0.13901041666666666667e-2_f64 * t7703 * t27772 * t100975 * t1003 + 0.16581944444444444444e-2_f64 * t100578 - t96019 - 0.13901041666666666667e-2_f64 * t7687 * t28952 + 0.37069444444444444445e-2_f64 * t7696 * t28952 - 0.46336805555555555557e-3_f64 * t101356 - 0.16581944444444444444e-2_f64 * t100580 - 0.44218518518518518516e-2_f64 * t100583 + 0.3684876543209876543e-2_f64 * t100586 + 0.69505208333333333333e-3_f64 * t7687 * t28948;
    (t101342, t101355, t101363)
}
