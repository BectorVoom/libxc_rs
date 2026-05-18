//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1235/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1235<F: Float>(t464: F, t6673: F, t132: F, t137: F, t477: F, t1629: F, t6734: F, t12245: F, t12248: F, t12259: F, t161: F, t166: F, t2088: F, t4801: F) -> (F, F, F, F, F, F) {
    let t16267 = t6673 * t464;
    let t16271 = t132 * t137 * t16267 * t477 / F::new(15.0);
    let t16275 = t132 * t137 * t6734 * t1629 / F::new(30.0);
    let t16276 = F::new(4.0) / F::new(135.0) * t12245;
    let t16277 = F::new(2.0) / F::new(45.0) * t12248;
    let t16278 = F::new(4.0) / F::new(45.0) * t12259;
    let t16282 = F::new(2.0) / F::new(15.0) * t161 * t166 * t4801 * t2088;
    (t16271, t16275, t16276, t16277, t16278, t16282)
}
