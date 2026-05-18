//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1077/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1077<F: Float>(t16776: F, t1966: F, t439: F, t822: F, t2002: F, t6255: F, t6259: F, t1989: F, t6134: F, t224: F, t7627: F, t500: F) -> (F, F, F, F, F) {
    let t19954 = t439 * t1966 * t16776 * t822 / F::new(5.0);
    let t19956 = F::new(3.0) / F::new(5.0) * t2002 * t6255;
    let t19958 = F::new(2.0) / F::new(5.0) * t2002 * t6259;
    let t19960 = t6134 * t1989 / F::new(15.0);
    let t19961 = t7627 * t224;
    let t19963 = t19961 * t500 / F::new(45.0);
    (t19954, t19956, t19958, t19960, t19963)
}
