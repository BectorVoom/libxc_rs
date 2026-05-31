//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1138/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1138<F: Float>(t1489: F, t165: F, t1994: F, t493: F, t1588: F, t1848: F, t3447: F, t831: F, t10267: F, t146: F, t4989: F, t9712: F) -> (F, F, F, F, F) {
    let t13525 = t493 * t165 * t1489 * t1994 / F::cast_from(5.0_f64);
    let t13527 = t1848 * t1588 / F::cast_from(10.0_f64);
    let t13529 = t831 * t3447 / F::cast_from(10.0_f64);
    let t13530 = t10267 / F::cast_from(45.0_f64);
    let t13532 = t146 * t9712 * t4989;
    (t13525, t13527, t13529, t13530, t13532)
}
