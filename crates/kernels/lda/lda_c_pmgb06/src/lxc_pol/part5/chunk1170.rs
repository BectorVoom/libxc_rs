//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1170/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1170<F: Float>(t1963: F, t6127: F, t17875: F, t14016: F, t14465: F, t14467: F, t14472: F, t21050: F, t21052: F, t21055: F, t21059: F, t21061: F) -> (F, F, F) {
    let t21065 = t6127 * t1963 / F::new(15.0);
    let t21066 = t17875 / F::new(15.0);
    let t21067 = t21050 - t21052 - t21055 - t21059 + F::new(12.0) * t14465 + F::new(4.0) / F::new(3.0) * t21061 + F::cast_from(0.0033101111111111113_f64) * t14467 + t14472 + t21065 - t21066 + t14016;
    (t21065, t21066, t21067)
}
