//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 969/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk969<F: Float>(t8370: F, t8374: F, t11441: F, t8353: F, t8358: F, t8376: F, t8379: F, t8382: F, t8386: F, t8388: F, t8390: F, t3631: F, t783: F) -> (F, F, F, F) {
    let t11443 = F::cast_from(3.8973666666666666_f64) * t8370;
    let t11444 = F::cast_from(1.9486833333333333_f64) * t8374;
    let t11451 = F::new(6.85552) * t8353 + t11441 + F::new(14.0) / F::new(9.0) * t8358 + t11443 - t11444 + F::new(11.75232) * t8376 + F::new(2.0) * t8379 + F::new(2.0) * t8382 + F::new(5.87616) * t8386 + F::new(5.87616) * t8388 - F::new(2.93808) * t8390;
    let t11465 = t783 * t3631;
    (t11443, t11444, t11451, t11465)
}
