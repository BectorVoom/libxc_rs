//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1112/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1112<F: Float>(t13215: F, t2012: F, t431: F, t5210: F, t1423: F, t5171: F, t1631: F, t1887: F, t3047: F, t802: F, t10040: F, t10046: F) -> (F, F, F, F, F, F, F) {
    let t13216 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13215;
    let t13218 = t431 * t5210 * t2012;
    let t13219 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13218;
    let t13220 = t1423 * t5171;
    let t13221 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13220;
    let t13223 = t1887 * t1631 / F::cast_from(10.0_f64);
    let t13225 = t802 * t3047 / F::cast_from(10.0_f64);
    let t13226 = t10040 / F::cast_from(15.0_f64);
    let t13227 = t10046 / F::cast_from(45.0_f64);
    (t13216, t13219, t13221, t13223, t13225, t13226, t13227)
}
