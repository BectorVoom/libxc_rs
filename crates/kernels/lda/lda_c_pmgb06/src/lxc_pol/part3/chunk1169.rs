//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1169/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1169<F: Float>(t10439: F, t3033: F, t439: F, t809: F, t2002: F, t2957: F, t2948: F, t5344: F, t1069: F, t1385: F, t1531: F, t2064: F) -> (F, F, F, F) {
    let t13958 = F::new(2.0) / F::new(15.0) * t439 * t10439 * t809 * t3033;
    let t13960 = t2002 * t2957 / F::new(15.0);
    let t13963 = F::new(2.0) / F::new(15.0) * t439 * t2948 * t5344;
    let t13968 = F::new(2.0) / F::new(15.0) * t439 * t1385 * t2064 * t1531 * t1069;
    (t13958, t13960, t13963, t13968)
}
