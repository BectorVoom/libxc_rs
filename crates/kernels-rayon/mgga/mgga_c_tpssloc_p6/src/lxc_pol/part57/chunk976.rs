//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 976/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk976(t1985: f64, t28205: f64, t31137: f64, t120632: f64, t22633: f64, t22635: f64, t31099: f64, t6347: f64, t26331: f64, t6330: f64, t6287: f64, t652: f64, t8326: f64) -> (f64, f64, f64, f64, f64) {
    let t127448 = 0.16449340668482264365e-1_f64 * t1985 * t31137 * t28205;
    let t127455 = 0.76763589786250567036e-1_f64 * t120632;
    let t127459 = 0.3289868133696452873e-1_f64 * t22633 * t22635 * t31099 * t6347;
    let t127463 = 0.9869604401089358619e-1_f64 * t26331 * t22635 * t31099 * t6330;
    let t127539 = 2.0_f64 * t652 * t6287 * t8326;
    (t127448, t127455, t127459, t127463, t127539)
}
