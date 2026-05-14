//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 908/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk908<F: Float>(t2120: F, t4568: F, t1518: F, t2504: F, t493: F, t3899: F, t571: F, t6969: F, t6973: F, t1294: F, t2402: F, t5175: F, t6875: F, t184: F, t202: F, t6669: F) -> (F, F, F, F, F, F, F) {
    let t17458 = t2120 * t4568;
    let t17461 = t493 * t1518 * t2504;
    let t17505 = t571 * t3899 * t6969;
    let t17508 = t571 * t3899 * t6973;
    let t17548 = t2402 * t1294;
    let t17550 = t6875 * t5175;
    let t17557 = t202 * t6669 * t184;
    (t17458, t17461, t17505, t17508, t17548, t17550, t17557)
}
