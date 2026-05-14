//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 700/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk700<F: Float>(t3044: F, t2089: F, t489: F, t161: F, t2100: F, t1600: F, t842: F, t1602: F, t166: F, t2018: F, t486: F, t2107: F, t435: F, t132: F, t1592: F, t813: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5101 = 2.0 / 45.0 * t3044;
    let t5102 = t489 * t2089;
    let t5104 = 2.0 / 45.0 * t161 * t5102;
    let t5105 = t489 * t2100;
    let t5107 = 2.0 / 45.0 * t161 * t5105;
    let t5108 = t842 * t1600;
    let t5109 = t5108 * t1602;
    let t5110 = t166 * t5109;
    let t5112 = t161 * t5110 / 15.0;
    let t5114 = 2.0 / 45.0 * t486 * t2018;
    let t5115 = t435 * t2107;
    let t5117 = 2.0 / 45.0 * t132 * t5115;
    let t5118 = t813 * t1592;
    (t5101, t5102, t5104, t5105, t5107, t5109, t5110, t5112, t5114, t5115, t5117, t5118)
}
