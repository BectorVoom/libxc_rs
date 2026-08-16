//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1241/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1241(t31193: f64, t5187: f64, t6637: f64, t6888: f64, t114064: f64, t120441: f64, t120445: f64, t120447: f64, t120452: f64, t120456: f64, t120459: f64, t120463: f64, t120467: f64, t120468: f64, t120469: f64, t120471: f64, t120475: f64, t120483: f64, t1336: f64, t1352: f64, t1814: f64, t31211: f64, t31212: f64, t31214: f64, t5230: f64, t5234: f64, t5287: f64, t8483: f64) -> f64 {
    let t120487 = 0.3289868133696452873e-1_f64 * t6888 * t6637 * t31193 * t5187;
    let t120488 = -t120475 * t1336 * t1352 - t1336 * t31211 * t5287 + t1814 * t31214 - t31212 * t5234 + t5230 * t8483 - t114064 + t120441 - t120445 + t120447 - t120452 - t120456 + t120459 + t120463 + t120467 + t120468 + t120469 + t120471 - t120483 - t120487;
    t120488
}
