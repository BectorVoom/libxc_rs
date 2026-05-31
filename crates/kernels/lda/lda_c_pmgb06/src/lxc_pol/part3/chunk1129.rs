//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1129/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1129<F: Float>(t13378: F, t13416: F, t161: F, t166: F, t176: F, t486: F, t5417: F, t3146: F, t844: F, t1499: F, t1837: F, t1417: F, t5305: F) -> (F, F, F, F, F) {
    let t13421 = t161 * t166 * (t13378 + t13416) * t176 / F::cast_from(30.0_f64);
    let t13423 = t486 * t5417 / F::cast_from(10.0_f64);
    let t13425 = t3146 * t844 / F::cast_from(30.0_f64);
    let t13427 = t1499 * t1837 / F::cast_from(10.0_f64);
    let t13429 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5305 * t1417;
    (t13421, t13423, t13425, t13427, t13429)
}
