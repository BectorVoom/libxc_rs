//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1229/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1229<F: Float>(t12191: F, t4841: F, t831: F, t12202: F, t1420: F, t6465: F, t10148: F, t439: F, t6464: F, t1629: F, t2570: F, t2960: F) -> (F, F, F, F, F, F) {
    let t16187 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12191;
    let t16189 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t831 * t4841;
    let t16190 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t12202;
    let t16192 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1420 * t6465;
    let t16195 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t439 * t10148 * t6464;
    let t16199 = t439 * t2960 * t2570 * t1629 / F::cast_from(27.0_f64);
    (t16187, t16189, t16190, t16192, t16195, t16199)
}
