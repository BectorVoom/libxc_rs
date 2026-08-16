//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1379/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1379(t1983: f64, t2019: f64, t74014: f64, t1390: f64, t2018: f64, t20356: f64, t1845: f64, t6330: f64, t24995: f64, t8643: f64, t28239: f64, t7685: f64) -> (f64, f64, f64, f64) {
    let t106964 = t1983 * t2019 * t74014;
    let t106968 = 6.0_f64 * t1983 * t20356 * t2018 * t1390;
    let t106971 = t6330 * t1845;
    let t106974 = 18.0_f64 * t24995 * t8643 * t106971;
    let t106978 = 3.0_f64 * t7685 * t28239;
    (t106964, t106968, t106974, t106978)
}
