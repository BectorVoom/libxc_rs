//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 994/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk994(t113070: f64, t113086: f64, t113124: f64, t114971: f64, t114977: f64, t114988: f64, t1877: f64, t2249: f64, t22960: f64, t22968: f64, t23299: f64, t23302: f64, t24191: f64, t24339: f64, t25: f64, t2522: f64, t25373: f64, t26563: f64, t26756: f64, t31430: f64, t31434: f64, t31449: f64, t31451: f64, t606: f64, t6542: f64, t7114: f64, t8566: f64, t92271: f64) -> f64 {
    let t114991 = -t1877 * t7114 * t113086 + t1877 * t8566 * t2249 / 2.0_f64 + 2.0_f64 * t92271 * t31449 - t1877 * t31434 * t23299 - t1877 * t31434 * t23302 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t22968 - 3.0_f64 * t24191 * t113070 + t1877 * t114971 * t25 / 2.0_f64 + t1877 * t31430 * t606 - 3.0_f64 * t26563 * t22960 * t114977 + 2.0_f64 * t26756 * t113124 - t1877 * t24339 * t31451 + 3.0_f64 * t2522 * t31430 * t6542 + t26756 * t25373 * t114988;
    t114991
}
