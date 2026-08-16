//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1331/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1331(t22635: f64, t26331: f64, t31099: f64, t5308: f64, t1385: f64, t1799: f64, t22633: f64, t31090: f64, t114285: f64, t26215: f64, t114226: f64, t1992: f64, t31091: f64, t90566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120239 = 0.9869604401089358619e-1_f64 * t26331 * t22635 * t31099 * t5308;
    let t120240 = t1799 * t1385;
    let t120244 = 0.6579736267392905746e-1_f64 * t22633 * t22635 * t31090 * t120240;
    let t120247 = 0.3289868133696452873e-1_f64 * t22633 * t114285 * t26215;
    let t120253 = 0.3289868133696452873e-1_f64 * t22633 * t22635 * t114226 * t1799;
    let t120258 = 0.3289868133696452873e-1_f64 * t1992 * t90566 * t31091;
    (t120239, t120240, t120244, t120247, t120253, t120258)
}
