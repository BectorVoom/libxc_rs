//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 921/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk921(t107: f64, t1180: f64, t2164: f64, t2786: f64, t902: f64, t4844: f64, t486: f64, t3005: f64, t831: f64, t1730: f64, t2025: f64, t2021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11744 = t107 * t1180 * t2164;
    let t11745 = 3.9861630686838536_f64 * t11744;
    let t11747 = t107 * t2786 * t902;
    let t11757 = t486 * t4844;
    let t11758 = t11757 / 45.0_f64;
    let t11777 = t831 * t3005;
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    (t11745, t11747, t11758, t11777, t11796, t11798)
}
