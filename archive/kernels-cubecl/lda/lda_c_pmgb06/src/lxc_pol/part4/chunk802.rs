//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 802/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk802<F: Float>(t2021: F, t97: F, t588: F, t1499: F, t844: F, t1837: F, t486: F, t4876: F, t2900: F, t2901: F, t2903: F, t2905: F, t2907: F, t4859: F, t4863: F, t4868: F, t4871: F, t4874: F, t4878: F, t4882: F, t4887: F, t4911: F, t4916: F, t4924: F) -> (F, F, F, F, F, F) {
    let t5391 = t2021 * t97;
    let t5393 = F::cast_from(0.12155555555555556_f64) * t5391 * t588;
    let t5396 = t1499 * t844 / F::cast_from(30.0_f64);
    let t5398 = t486 * t1837 / F::cast_from(15.0_f64);
    let t5405 = F::cast_from(0.002518888888888889_f64) * t4876;
    let t5415 = t2900 + F::cast_from(0.0016792592592592592_f64) * t2901 - F::cast_from(0.0004198148148148148_f64) * t2903 + F::cast_from(0.0012594444444444445_f64) * t2905 - F::cast_from(0.0006297222222222223_f64) * t2907 + F::cast_from(0.0008396296296296296_f64) * t4911 - F::cast_from(0.0008396296296296296_f64) * t4878 + t5405 + F::cast_from(0.01385388888888889_f64) * t4916 + F::cast_from(0.002099074074074074_f64) * t4887 - F::cast_from(0.007556666666666666_f64) * t4859 - F::cast_from(0.005037777777777778_f64) * t4868 + F::cast_from(0.0012594444444444445_f64) * t4882 + F::cast_from(0.011335_f64) * t4863 + F::cast_from(0.015113333333333333_f64) * t4874 - F::cast_from(0.003778333333333333_f64) * t4871 - F::cast_from(0.003778333333333333_f64) * t4924;
    (t5391, t5393, t5396, t5398, t5405, t5415)
}
