//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1323/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1323(t110111: f64, t110141: f64, t110144: f64, t110146: f64, t110158: f64, t110520: f64, t110521: f64, t110526: f64, t110531: f64, t110533: f64, t110542: f64, t1444: f64, t2: f64, t29907: f64, t29911: f64, t29922: f64, t30175: f64, t4049: f64, t4067: f64, t8128: f64, t8137: f64) -> f64 {
    let t110549 = -20.0_f64 / 9.0_f64 * t110111 - 5.0_f64 / 2.0_f64 * t110520 * t110521 * t29911 + 5.0_f64 / 9.0_f64 * t110526 * t4049 * t29911 - t110531 + 125.0_f64 / 72.0_f64 * t110533 - 25.0_f64 / 27.0_f64 * t8137 * t110158 * t1444 + 25.0_f64 / 36.0_f64 * t30175 * t29922 * t2 - t110542 - 5.0_f64 / 6.0_f64 * t8128 * t29907 * t4067 + 44.0_f64 / 9.0_f64 * t110141 - 110.0_f64 / 27.0_f64 * t110144 - 2.0_f64 / 3.0_f64 * t110146;
    t110549
}
