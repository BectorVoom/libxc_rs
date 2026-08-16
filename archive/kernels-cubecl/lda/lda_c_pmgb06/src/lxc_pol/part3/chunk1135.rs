//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1135/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1135<F: Float>(t493: F, t5179: F, t5318: F, t1586: F, t1992: F, t5174: F, t4612: F, t5168: F, t2010: F, t2011: F, t3216: F, t1447: F, t5313: F) -> (F, F, F, F, F) {
    let t13492 = t493 * t5179 * t5318 / F::cast_from(5.0_f64);
    let t13496 = t493 * t1992 * t5174 * t1586 / F::cast_from(5.0_f64);
    let t13498 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5168 * t4612;
    let t13501 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2010 * t3216 * t2011;
    let t13502 = t1447 * t5313;
    (t13492, t13496, t13498, t13501, t13502)
}
