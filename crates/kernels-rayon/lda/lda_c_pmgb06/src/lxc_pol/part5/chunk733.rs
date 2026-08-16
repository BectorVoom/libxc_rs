//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 733/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk733(t1476: f64, t6765: f64, t36: f64, t506: f64, t6760: f64, t497: f64, t5974: f64, t2900: f64, t2901: f64, t4878: f64, t4911: f64, t4916: f64, t5405: f64, t6800: f64, t6803: f64, t6806: f64, t6809: f64, t6811: f64, t6814: f64, t6817: f64, t6819: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6821 = t1476 * t6765;
    let t6822 = t36 * t6821;
    let t6824 = t506 * t6760;
    let t6825 = t36 * t6824;
    let t6827 = t497 * t5974;
    let t6828 = t506 * t6827;
    let t6829 = t36 * t6828;
    let t6831 = t2900 + 0.0008396296296296296_f64 * t2901 + 0.0016792592592592592_f64 * t4911 - 0.0008396296296296296_f64 * t4878 + t5405 + 0.002518888888888889_f64 * t4916 - 0.0004198148148148148_f64 * t6800 + 0.002099074074074074_f64 * t6803 - 0.007556666666666666_f64 * t6806 - 0.005037777777777778_f64 * t6809 + 0.0012594444444444445_f64 * t6811 + 0.011335_f64 * t6814 + 0.015113333333333333_f64 * t6817 - 0.0006297222222222223_f64 * t6819 + 0.0012594444444444445_f64 * t6822 - 0.003778333333333333_f64 * t6825 + 0.0018891666666666666_f64 * t6829;
    (t6821, t6822, t6824, t6825, t6827, t6828, t6829, t6831)
}
