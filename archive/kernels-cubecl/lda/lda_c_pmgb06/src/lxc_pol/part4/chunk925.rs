//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 925/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk925<F: Float>(t1476: F, t6765: F, t36: F, t506: F, t6760: F, t497: F, t5974: F, t2900: F, t2901: F, t4878: F, t4911: F, t4916: F, t5405: F, t6800: F, t6803: F, t6806: F, t6809: F, t6811: F, t6814: F, t6817: F, t6819: F) -> (F, F, F, F, F, F, F, F) {
    let t6821 = t1476 * t6765;
    let t6822 = t36 * t6821;
    let t6824 = t506 * t6760;
    let t6825 = t36 * t6824;
    let t6827 = t497 * t5974;
    let t6828 = t506 * t6827;
    let t6829 = t36 * t6828;
    let t6831 = t2900 + F::cast_from(0.0008396296296296296_f64) * t2901 + F::cast_from(0.0016792592592592592_f64) * t4911 - F::cast_from(0.0008396296296296296_f64) * t4878 + t5405 + F::cast_from(0.002518888888888889_f64) * t4916 - F::cast_from(0.0004198148148148148_f64) * t6800 + F::cast_from(0.002099074074074074_f64) * t6803 - F::cast_from(0.007556666666666666_f64) * t6806 - F::cast_from(0.005037777777777778_f64) * t6809 + F::cast_from(0.0012594444444444445_f64) * t6811 + F::cast_from(0.011335_f64) * t6814 + F::cast_from(0.015113333333333333_f64) * t6817 - F::cast_from(0.0006297222222222223_f64) * t6819 + F::cast_from(0.0012594444444444445_f64) * t6822 - F::cast_from(0.003778333333333333_f64) * t6825 + F::cast_from(0.0018891666666666666_f64) * t6829;
    (t6821, t6822, t6824, t6825, t6827, t6828, t6829, t6831)
}
