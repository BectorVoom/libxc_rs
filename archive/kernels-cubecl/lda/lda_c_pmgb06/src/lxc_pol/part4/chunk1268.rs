//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1268/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1268<F: Float>(t439: F, t5225: F, t6160: F, t15445: F, t1897: F, t15353: F, t15358: F, t1901: F, t1420: F, t6419: F, t5253: F, t6165: F) -> (F, F, F, F, F, F) {
    let t16662 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t5225 * t6160;
    let t16665 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t1897 * t15445;
    let t16668 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t439 * t1897 * t15353;
    let t16671 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t439 * t1901 * t15358;
    let t16673 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1420 * t6419;
    let t16676 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t439 * t5253 * t6165;
    (t16662, t16665, t16668, t16671, t16673, t16676)
}
