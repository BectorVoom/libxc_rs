//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 982/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk982<F: Float>(t1967: F, t2497: F, t4488: F, t4500: F, t2131: F, t6597: F, t11900: F, t2329: F, t806: F, t348: F, t4494: F, t4501: F, t20729: F, t3965: F, t5141: F, t3967: F, t494: F) -> (F, F, F, F, F, F, F, F) {
    let t20773 = 4.0 / 9.0 * t4488 * t4500 * t2497 * t1967;
    let t20775 = 4.0 / 5.0 * t6597 * t2131;
    let t20776 = 8.0 / 45.0 * t11900;
    let t20777 = t2329 * t806;
    let t20778 = t20777 * t348;
    let t20781 = 8.0 / 15.0 * t4488 * t4494 * t20778;
    let t20784 = 4.0 / 9.0 * t4488 * t4501 * t20778;
    let t20787 = 16.0 / 15.0 * t3965 * t5141 * t20729;
    let t20791 = 8.0 / 15.0 * t3965 * t3967 * t20777 * t494;
    (t20773, t20775, t20776, t20777, t20781, t20784, t20787, t20791)
}
