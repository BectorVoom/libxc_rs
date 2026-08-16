//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 737/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk737<F: Float>(t525: F, t6406: F, t6760: F, t1576: F, t6765: F, t103: F, t2060: F, t3368: F, t3380: F, t6800: F, t6811: F, t6819: F, t6829: F, t6873: F, t6875: F, t6877: F, t6879: F, t6882: F, t6885: F, t6888: F, t6891: F) -> (F, F, F, F) {
    let t6894 = t525 * t6406;
    let t6897 = t525 * t6760;
    let t6900 = t1576 * t6765;
    let t6903 = -t3368 - t3380 + F::cast_from(0.007998148148148148_f64) * t6800 - F::cast_from(0.023994444444444443_f64) * t6811 + F::cast_from(0.011997222222222222_f64) * t6819 - F::cast_from(0.035991666666666665_f64) * t6829 - F::cast_from(0.008888888888888889_f64) * t6873 + F::cast_from(0.0044444444444444444_f64) * t6875 + F::cast_from(0.0014814814814814814_f64) * t6877 - F::cast_from(0.006666666666666667_f64) * t103 * t6879 + F::cast_from(0.013333333333333334_f64) * t103 * t6882 - F::cast_from(0.002962962962962963_f64) * t103 * t6885 + F::cast_from(0.008888888888888889_f64) * t2060 * t6888 - F::cast_from(0.04_f64) * t103 * t6891 - F::cast_from(0.05333333333333334_f64) * t2060 * t6894 + F::cast_from(0.013333333333333334_f64) * t103 * t6897 - F::cast_from(0.0022222222222222222_f64) * t103 * t6900;
    (t6894, t6897, t6900, t6903)
}
