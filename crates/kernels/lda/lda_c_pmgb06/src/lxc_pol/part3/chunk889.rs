//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 889/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk889<F: Float>(t9352: F, t9379: F, t9381: F, t9383: F, t9385: F, t9393: F, t9395: F, t132: F, t1547: F, t2065: F, t1381: F, t1601: F, t2088: F, t5068: F, t130: F, t485: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11889 = 4.0 / 135.0 * t9352;
    let t11890 = 2.0 / 135.0 * t9379;
    let t11891 = 2.0 / 81.0 * t9381;
    let t11892 = 4.0 / 27.0 * t9383;
    let t11893 = 2.0 / 27.0 * t9385;
    let t11894 = 2.0 / 45.0 * t9393;
    let t11895 = 2.0 / 15.0 * t9395;
    let t11897 = t132 * t1547 * t2065;
    let t11898 = t11897 / 45.0;
    let t11902 = 4.0 / 15.0 * t5068 * t1601 * t2088 * t1381;
    let t11903 = t485 * t130;
    (t11889, t11890, t11891, t11892, t11893, t11894, t11895, t11898, t11902, t11903)
}
