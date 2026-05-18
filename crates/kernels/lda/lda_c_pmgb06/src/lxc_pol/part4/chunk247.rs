//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 247/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk247<F: Float>(t12: F, t336: F, t764: F, t763: F, zeta_threshold: F) -> (F, F) {
    let t13 = t12 <= zeta_threshold;
    let t765 = t336 * t764;
    let t767 = piecewise3::<f64>(t13, F::new(0.0), F::new(2.0) / F::new(3.0) * t765);
    let t769 = t763 / F::new(2.0) + t767 / F::new(2.0);
    (t765, t769)
}
