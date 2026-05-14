//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 836/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk836<F: Float>(t11862: F, t129: F, t2012: F, t10318: F, t806: F, t1600: F, t1988: F, t1898: F, t3213: F, t161: F, t3004: F, t843: F, t132: F, t1547: F, t2065: F, t432: F, t5051: F) -> (F, F, F, F, F, F, F) {
    let t11864 = t129 * t11862 * t2012;
    let t11866 = t10318 * t806;
    let t11867 = 2.0 / 135.0 * t11866;
    let t11877 = t1988 * t1600;
    let t11881 = t3213 * t1898;
    let t11882 = 4.0 / 135.0 * t11881;
    let t11884 = t161 * t3004 * t843;
    let t11897 = t132 * t1547 * t2065;
    let t11898 = t11897 / 45.0;
    let t11914 = t432 * t5051;
    (t11864, t11867, t11877, t11882, t11884, t11898, t11914)
}
