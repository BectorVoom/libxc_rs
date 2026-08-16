//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 720/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk720(t127: f64, t4615: f64, t338: f64, t3851: f64, t797: f64, t874: f64, t3814: f64, t837: f64, t892: f64, t1318: f64, t21: f64, t41: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25809 = 1.0_f64 / t4615 / t127;
    let t25820 = t3851 * t338;
    let t25854 = t797 * t874;
    let t25877 = t3814 * t338;
    let t25918 = t892 * t837;
    let t26004 = t1318 * t1318;
    let t26007 = t21 / t41 / t26004;
    (t25809, t25820, t25854, t25877, t25918, t26007)
}
