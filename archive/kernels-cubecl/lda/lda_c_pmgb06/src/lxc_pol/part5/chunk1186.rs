//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1186/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1186<F: Float>(t11402: F, t342: F, t7344: F, t5874: F, t8306: F, t8355: F, t11234: F, t38: F, t7321: F, t2209: F, t2703: F, t2221: F, t2448: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21409 = F::cast_from(3.8973666666666666_f64) * t11402;
    let t21410 = t7344 * t342;
    let t21411 = t5874 * t21410;
    let t21414 = F::cast_from(1.2991222222222223_f64) * t8306;
    let t21416 = F::cast_from(1.5156425925925925_f64) * t8355;
    let t21423 = F::cast_from(52.61445_f64) * t11234 * t21410;
    let t21439 = F::cast_from(70.1526_f64) * t38 * t7321 * t342;
    let t21442 = F::cast_from(52.61445_f64) * t38 * t2703 * t2209;
    let t21445 = F::cast_from(17.53815_f64) * t38 * t2221 * t2448;
    (t21409, t21410, t21411, t21414, t21416, t21423, t21439, t21442, t21445)
}
