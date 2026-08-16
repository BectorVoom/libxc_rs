//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1306/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1306(t1983: f64, t2019: f64, t74064: f64, t28813: f64, t7685: f64, t28821: f64, t7754: f64, t1845: f64, t6463: f64, t26161: f64, t26162: f64, t1799: f64, t6324: f64) -> (f64, f64, f64, f64, f64) {
    let t105184 = 6.0_f64 * t1983 * t2019 * t74064;
    let t105186 = 6.0_f64 * t7685 * t28813;
    let t105188 = 3.0_f64 * t28821 * t7754;
    let t105189 = t6463 * t1845;
    let t105192 = 6.0_f64 * t26161 * t26162 * t105189;
    let t105201 = t1799 * t6324;
    (t105184, t105186, t105188, t105192, t105201)
}
