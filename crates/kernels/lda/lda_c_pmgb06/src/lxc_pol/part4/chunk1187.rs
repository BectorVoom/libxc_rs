//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1187/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1187<F: Float>(t5041: F, t802: F, t4975: F, t486: F, t6232: F, t161: F, t1639: F, t166: F, t6904: F, t132: F, t435: F, t6226: F, t14211: F, t14213: F, t1600: F, t1992: F, t493: F, t529: F) -> (F, F, F, F, F, F, F, F) {
    let t17950 = t802 * t5041 / 15.0;
    let t17952 = 2.0 / 15.0 * t802 * t4975;
    let t17954 = t486 * t6232 / 15.0;
    let t17958 = t161 * t166 * t1639 * t6904 / 15.0;
    let t17960 = t132 * t435 * t6226;
    let t17961 = 2.0 / 45.0 * t17960;
    let t17962 = 4.0 / 135.0 * t14211;
    let t17963 = 4.0 / 45.0 * t14213;
    let t17964 = t1600 * t6904;
    let t17968 = 2.0 / 15.0 * t493 * t1992 * t17964 * t529;
    (t17950, t17952, t17954, t17958, t17961, t17962, t17963, t17968)
}
