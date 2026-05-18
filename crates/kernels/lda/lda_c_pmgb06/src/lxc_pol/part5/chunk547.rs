//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 547/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk547<F: Float>(t154: F, t2851: F, t132: F, t1548: F, t432: F, t1547: F, t459: F, t1592: F, t442: F, t1600: F, t496: F, t1179: F, t139: F) -> (F, F, F, F, F, F, F, F) {
    let t2852 = t2851 * t154;
    let t2854 = F::new(4.0) / F::new(405.0) * t132 * t2852;
    let t2855 = t432 * t1548;
    let t2857 = t1547 * t459;
    let t2858 = t132 * t2857;
    let t2864 = t442 * t1592;
    let t2871 = t496 * t1600;
    let t2897 = t1179 * t139;
    (t2852, t2854, t2855, t2857, t2858, t2864, t2871, t2897)
}
