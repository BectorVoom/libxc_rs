//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1013/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1013(t225: f64, t33940: f64, t114762: f64, t116514: f64, t121382: f64, t121391: f64, t121399: f64, t121403: f64, t121409: f64, t121451: f64, t2053: f64, t25168: f64, t2597: f64, t26581: f64, t26679: f64, t26700: f64, t26728: f64, t26729: f64, t2713: f64, t2718: f64, t32006: f64, t33982: f64, t4147: f64, t7092: f64, t855: f64, t866: f64) -> f64 {
    let t123487 = t33940 * t225;
    let t123503 = 2.0_f64 * t4147 * t32006 + 4.0_f64 * t26700 * t7092 - 12.0_f64 * t25168 * t26728 * t26581 + t116514 - 0.15352717957250113407e0_f64 * t114762 + 0.6579736267392905746e-1_f64 * t121382 - t123487 * t866 + 2.0_f64 * t2597 * t33982 + 0.3289868133696452873e-1_f64 * t121391 - 12.0_f64 * t121451 * t26729 + 4.0_f64 * t855 * t2718 * t2053 * t26679 + 0.16449340668482264365e-1_f64 * t121399 - 0.6579736267392905746e-1_f64 * t121403 + 2.0_f64 * t2713 * t33982 - 0.3289868133696452873e-1_f64 * t121409;
    t123503
}
