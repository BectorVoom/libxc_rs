//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1012/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1012(t10109: f64, t8740: f64, t114613: f64, t114615: f64, t121336: f64, t121339: f64, t121352: f64, t121362: f64, t121367: f64, t121371: f64, t13053: f64, t13065: f64, t13463: f64, t24305: f64, t25168: f64, t2597: f64, t2713: f64, t32018: f64, t33935: f64, t33951: f64, t4147: f64, t4272: f64, t7830: f64, t8741: f64) -> f64 {
    let t123464 = t10109 * t8740;
    let t123476 = -t13053 * t8741 - t13065 * t8741 + 0.6579736267392905746e-1_f64 * t121336 + 0.19739208802178717238e0_f64 * t121339 - t13463 * t8741 + 0.6579736267392905746e-1_f64 * t121352 - 0.3289868133696452873e-1_f64 * t114613 - 0.76763589786250567037e-1_f64 * t114615 - 0.19739208802178717238e0_f64 * t121362 + 4.0_f64 * t24305 * t7830 - 6.0_f64 * t25168 * t123464 * t4272 + 0.6579736267392905746e-1_f64 * t121367 - 6.0_f64 * t2713 * t33951 - 6.0_f64 * t4147 * t32018 - 0.15352717957250113407e0_f64 * t121371 + 4.0_f64 * t2597 * t33935;
    t123476
}
