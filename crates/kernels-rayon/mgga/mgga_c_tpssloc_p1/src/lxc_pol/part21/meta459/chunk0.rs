//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2021/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2021(t15968: f64, t182: f64, t1787: f64, t2516: f64, t17: f64, t12097: f64, t12100: f64, t12111: f64, t12120: f64, t184: f64, t2663: f64, t5157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15970 = 0.19751673498613801407e-1_f64 * t15968 * t182;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15973 = 0.4883052614935078681e-3_f64 * t12097;
    let t15974 = 0.18311447306006545054e-3_f64 * t12100;
    let t15975 = 0.21687162600603479684e-1_f64 * t12111;
    let t15976 = 4.0_f64 * t12120;
    let t15977 = t15968 * t184;
    let t15978 = t17 * t15977;
    let t15979 = t5157 * t2663;
    (t15970, t15971, t15972, t15973, t15974, t15975, t15976, t15977, t15978, t15979)
}
