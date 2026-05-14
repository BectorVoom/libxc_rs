//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1035/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1035<F: Float>(t17423: F, t1325: F, t2437: F, t34: F, t4829: F, t4738: F, t6433: F, t17426: F, t18555: F, t2480: F, t6867: F, t6875: F, t17434: F, t473: F, t16144: F, t2479: F, t266: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21564 = 16.0 / 45.0 * t17423;
    let t21568 = 8.0 / 15.0 * t1325 * t4829 * t2437 * t34;
    let t21570 = 16.0 / 5.0 * t4738 * t6433;
    let t21571 = 4.0 / 15.0 * t17426;
    let t21573 = 4.0 / 5.0 * t18555 * t2480;
    let t21575 = 4.0 / 5.0 * t6875 * t6867;
    let t21576 = 8.0 / 45.0 * t17434;
    let t21577 = t34 * t473;
    let t21581 = 4.0 / 5.0 * t21577 * t16144 * t266 * t2479;
    (t21564, t21568, t21570, t21571, t21573, t21575, t21576, t21577, t21581)
}
