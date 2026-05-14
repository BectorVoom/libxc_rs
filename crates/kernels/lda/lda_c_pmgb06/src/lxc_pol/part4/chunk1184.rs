//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1184/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1184<F: Float>(t1420: F, t6775: F, t2002: F, t5233: F, t2497: F, t3223: F, t1380: F, t1831: F, t1981: F, t2088: F, t14478: F, t17884: F, t17885: F, t17887: F, t17889: F, t17891: F, t17895: F, t17898: F, t17902: F, t17904: F) -> (F, F, F, F, F) {
    let t17906 = 2.0 / 45.0 * t1420 * t6775;
    let t17908 = 4.0 / 45.0 * t2002 * t5233;
    let t17909 = t3223 * t2497;
    let t17910 = 4.0 / 405.0 * t17909;
    let t17914 = 8.0 / 45.0 * t1981 * t1380 * t1831 * t2088;
    let t17916 = -t17884 - t17885 - t17887 + t17889 + t17891 + t17895 + t17898 + t17902 - t17904 - t17906 - t17908 + t17910 + t17914 + 8.0 / 3.0 * t14478;
    (t17906, t17908, t17910, t17914, t17916)
}
