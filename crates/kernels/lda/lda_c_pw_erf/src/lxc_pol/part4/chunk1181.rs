//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1181/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1181<F: Float>(t10164: F, t10167: F, t12951: F, t4647: F, t808: F, t2505: F, t5065: F, t5069: F, t2114: F, t6592: F, t4589: F, t2120: F, t4568: F, t1518: F, t2504: F, t493: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17444 = 16.0 / 135.0 * t10164;
    let t17445 = 64.0 / 1215.0 * t10167;
    let t17446 = 8.0 / 45.0 * t12951;
    let t17448 = 4.0 / 15.0 * t4647 * t808;
    let t17450 = 4.0 / 15.0 * t5065 * t2505;
    let t17452 = 8.0 / 15.0 * t5069 * t2505;
    let t17454 = 8.0 / 15.0 * t2114 * t6592;
    let t17457 = 8.0 / 15.0 * t4589 * t808;
    let t17458 = t2120 * t4568;
    let t17459 = 8.0 / 9.0 * t17458;
    let t17461 = t493 * t1518 * t2504;
    (t17444, t17445, t17446, t17448, t17450, t17452, t17454, t17457, t17459, t17461)
}
