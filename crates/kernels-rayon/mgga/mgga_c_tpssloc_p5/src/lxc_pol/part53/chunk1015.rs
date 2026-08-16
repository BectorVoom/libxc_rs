//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1015/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1015(t114827: f64, t114882: f64, t116557: f64, t116578: f64, t121467: f64, t121469: f64, t1527: f64, t25168: f64, t259: f64, t26702: f64, t26728: f64, t2713: f64, t2718: f64, t31964: f64, t31998: f64, t32006: f64, t33935: f64, t33947: f64, t4268: f64, t4273: f64, t4300: f64, t4301: f64, t7106: f64, t7841: f64, t798: f64, t855: f64, t8740: f64) -> f64 {
    let t123552 = -0.3289868133696452873e-1_f64 * t121467 + 0.15352717957250113407e0_f64 * t121469 - 0.16449340668482264365e-1_f64 * t114827 - t31964 * t4301 + 2.0_f64 * t31964 * t4273 + t798 * t33947 * t259 + 4.0_f64 * t2713 * t33935 - 12.0_f64 * t25168 * t26728 * t26702 - t116557 + 2.0_f64 * t855 * t2718 * t8740 * t4300 + 0.76763589786250567037e-1_f64 * t114882 + 4.0_f64 * t855 * t2718 * t7106 * t7841 + t116578 + 2.0_f64 * t4268 * t32006 + 2.0_f64 * t855 * t2718 * t31998 * t1527;
    t123552
}
