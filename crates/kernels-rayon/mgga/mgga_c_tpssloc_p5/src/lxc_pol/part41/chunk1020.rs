//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1020/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1020(t12045: f64, t12052: f64, t12054: f64, t5151: f64, t750: f64, t17: f64, t1787: f64, t2516: f64, t12120: f64, t2663: f64, t5157: f64, t1788: f64, t2225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15911 = 48.0_f64 * t12045;
    let t15916 = 12.0_f64 * t12052;
    let t15917 = 80.0_f64 * t12054;
    let t15921 = t5151 * t750;
    let t15923 = 2.0_f64 * t17 * t15921;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15976 = 4.0_f64 * t12120;
    let t15979 = t5157 * t2663;
    let t15982 = t2225 * t1788;
    (t15911, t15916, t15917, t15923, t15972, t15976, t15979, t15982)
}
