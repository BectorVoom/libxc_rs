//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1168/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1168(t1420: f64, t7547: f64, t439: f64, t5253: f64, t7485: f64, t1901: f64, t19758: f64, t17859: f64, t17861: f64, t21013: f64, t21016: f64, t21021: f64, t21026: f64, t21028: f64, t21033: f64, t21036: f64) -> (f64, f64, f64, f64) {
    let t21038 = t1420 * t7547 / 9.0_f64;
    let t21041 = t439 * t5253 * t7485 / 9.0_f64;
    let t21044 = t439 * t1901 * t19758 / 9.0_f64;
    let t21045 = t21013 - t21016 - t21021 - t21026 - t21028 + 4.0_f64 * t17859 + 8.0_f64 * t17861 - t21033 - t21036 + t21038 + t21041 + t21044;
    (t21038, t21041, t21044, t21045)
}
