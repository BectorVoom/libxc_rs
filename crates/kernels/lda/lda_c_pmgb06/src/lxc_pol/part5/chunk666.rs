//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 666/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk666<F: Float>(t342: F, t787: F, t374: F, t2695: F, t73: F, t388: F, t1316: F, t1324: F, t2180: F, t2255: F, t2308: F, t2733: F, t346: F, t384: F, t3987: F, t3991: F, t3995: F, t3999: F, t4005: F, t4355: F, t4360: F, t5583: F, t5999: F, t6006: F, t6009: F, t6013: F, t6018: F, t6021: F, t790: F) -> (F, F, F, F) {
    let t6024 = t787 * t342;
    let t6028 = t787 * t374;
    let t6031 = t73 * t2695;
    let t6032 = t388 * t6031;
    let t6035 = t346 * t790 * t2255 + F::new(12.0) * t2180 * t5999 + t346 * t2733 * t384 - F::new(0.0005811348303577384) * t3987 - t3991 + F::new(0.001355981270834723) * t3995 + t3999 - t4005 + F::new(2.0) * t6006 * t6009 - F::new(3.0) * t5583 * t6013 - F::new(6.0) * t5583 * t4355 + F::new(12.0) * t6018 * t4360 - t346 * t6021 * t1324 + F::new(3.0) * t1316 * t790 * t6024 - t346 * t2308 * t6028 + F::new(6.0) * t2180 * t6032;
    (t6024, t6028, t6031, t6035)
}
