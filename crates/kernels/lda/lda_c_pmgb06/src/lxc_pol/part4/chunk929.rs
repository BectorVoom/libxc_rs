//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 929/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk929<F: Float>(t525: F, t6406: F, t6760: F, t1576: F, t6765: F, t103: F, t2060: F, t3368: F, t3380: F, t6800: F, t6811: F, t6819: F, t6829: F, t6873: F, t6875: F, t6877: F, t6879: F, t6882: F, t6885: F, t6888: F, t6891: F) -> (F, F, F, F) {
    let t6894 = t525 * t6406;
    let t6897 = t525 * t6760;
    let t6900 = t1576 * t6765;
    let t6903 = -t3368 - t3380 + F::new(0.007998148148148148) * t6800 - F::new(0.023994444444444443) * t6811 + F::new(0.011997222222222222) * t6819 - F::new(0.035991666666666665) * t6829 - F::new(0.008888888888888889) * t6873 + F::new(0.0044444444444444444) * t6875 + F::new(0.0014814814814814814) * t6877 - F::new(0.006666666666666667) * t103 * t6879 + F::new(0.013333333333333334) * t103 * t6882 - F::new(0.002962962962962963) * t103 * t6885 + F::new(0.008888888888888889) * t2060 * t6888 - F::new(0.04) * t103 * t6891 - F::new(0.05333333333333334) * t2060 * t6894 + F::new(0.013333333333333334) * t103 * t6897 - F::new(0.0022222222222222222) * t103 * t6900;
    (t6894, t6897, t6900, t6903)
}
