//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 956/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk956<F: Float>(t2229: F, t3588: F, t38: F, t1234: F, t2233: F, t3559: F, t776: F, t247: F, t28: F, t769: F, t8276: F, t3615: F, t63: F) -> (F, F, F, F, F, F) {
    let t11211 = F::new(70.1526) * t38 * t2229 * t3588;
    let t11222 = F::new(52.61445) * t38 * t2233 * t1234;
    let t11225 = F::new(5.84605) * t38 * t776 * t3559;
    let t11227 = t769 * t28 * t247;
    let t11228 = t8276 * t11227;
    let t11229 = F::cast_from(1.9486833333333333_f64) * t11228;
    let t11230 = t63 * t3615;
    (t11211, t11222, t11225, t11227, t11229, t11230)
}
