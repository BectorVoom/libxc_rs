//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 646/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk646<F: Float>(t1351: F, t5165: F, t2066: F, t514: F, t211: F, t2071: F, t4567: F, t548: F, t1397: F, t2076: F, t2099: F, t185: F) -> (F, F, F, F, F, F, F, F) {
    let t5166 = t5165 * t1351;
    let t5170 = t514 * t2066;
    let t5172 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t211 * t5170;
    let t5175 = t4567 * t2071;
    let t5176 = t548 * t5175;
    let t5179 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2076 * t1397;
    let t5184 = t514 * t2099;
    let t5186 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t185 * t5184;
    (t5166, t5170, t5172, t5175, t5176, t5179, t5184, t5186)
}
