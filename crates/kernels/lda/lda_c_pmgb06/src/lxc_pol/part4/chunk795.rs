//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 795/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk795<F: Float>(t374: F, t783: F, t6007: F, t342: F, t4232: F, t1233: F, t5582: F, t1322: F, t2732: F, t787: F, t2695: F, t73: F, t388: F, t1316: F, t1324: F, t2180: F, t2255: F, t2308: F, t2733: F, t346: F, t384: F, t3987: F, t3991: F, t3995: F, t3999: F, t4005: F, t4355: F, t4360: F, t5583: F, t5999: F, t6006: F, t790: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6008 = t783 * t374;
    let t6009 = t6007 * t6008;
    let t6012 = t783 * t342;
    let t6013 = t4232 * t6012;
    let t6018 = t1233 * t5582;
    let t6021 = t2732 * t1322;
    let t6024 = t787 * t342;
    let t6028 = t787 * t374;
    let t6031 = t73 * t2695;
    let t6032 = t388 * t6031;
    let t6035 = t346 * t790 * t2255 + 12.0 * t2180 * t5999 + t346 * t2733 * t384 - 0.0005811348303577384 * t3987 - t3991 + 0.001355981270834723 * t3995 + t3999 - t4005 + 2.0 * t6006 * t6009 - 3.0 * t5583 * t6013 - 6.0 * t5583 * t4355 + 12.0 * t6018 * t4360 - t346 * t6021 * t1324 + 3.0 * t1316 * t790 * t6024 - t346 * t2308 * t6028 + 6.0 * t2180 * t6032;
    (t6008, t6009, t6012, t6013, t6018, t6021, t6024, t6028, t6031, t6032, t6035)
}
