//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1047/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1047(t2973: f64, t3: f64, t3917: f64, t668: f64, t26: f64, t1232: f64, t2950: f64, t1181: f64, t19: f64, t2949: f64, t2970: f64, t2972: f64, t2974: f64, t3115: f64, t3119: f64, t7835: f64, t7842: f64, t7851: f64, t7866: f64, t7868: f64, t9825: f64, t9827: f64, t9829: f64, t9834: f64, t9839: f64) -> (f64, f64, f64, f64, f64) {
    let t9846 = t2973 * t3;
    let t9850 = t3917 * t668;
    let t9851 = t26 * t9850;
    let t9858 = t2950 * t1232;
    let t9861 = -t9825 / 64.0_f64 - t9827 / 32.0_f64 - t7851 - t2970 * t9829 * t2974 / 24.0_f64 - t2970 * t2972 * t9834 / 48.0_f64 + t7842 * t2972 * t9839 / 16.0_f64 - 7.0_f64 / 144.0_f64 * t7866 * t7868 * t9839 - t2970 * t7835 * t9846 / 12.0_f64 - 3.0_f64 / 64.0_f64 * t19 * t9851 - 3.0_f64 / 32.0_f64 * t1181 * t3115 - 3.0_f64 / 32.0_f64 * t1181 * t3119 - 3.0_f64 / 16.0_f64 * t2949 * t9858;
    (t9846, t9850, t9851, t9858, t9861)
}
