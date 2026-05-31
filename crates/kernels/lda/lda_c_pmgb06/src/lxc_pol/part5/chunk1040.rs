//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1040/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1040<F: Float>(t2002: F, t6524: F, t1423: F, t7711: F, t11861: F, t11867: F, t19431: F, t19434: F, t19436: F, t19438: F, t19440: F, t19441: F, t19442: F, t19447: F) -> (F, F, F) {
    let t19449 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t6524;
    let t19450 = t1423 * t7711;
    let t19451 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t19450;
    let t19452 = -t19431 - t19434 + t19436 + t19438 + t19440 + t19441 - t11861 - t19442 - t11867 - t19447 + t19449 + t19451;
    (t19449, t19451, t19452)
}
