//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1144/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1144<F: Float>(t20747: F, t27: F, t545: F, t7704: F, t17544: F, t20684: F, t20689: F, t20692: F, t20694: F, t20739: F, t20741: F, t20745: F, t20746: F) -> (F, F) {
    let t20748 = F::new(2.0) / F::new(15.0) * t20747;
    let t20750 = t7704 * t27 * t545;
    let t20753 = t20684 + t20689 + t20692 + t20694 + t20739 - t20741 - t20745 - t20746 - t20748 + F::cast_from(0.10821041362364843_f64) * t20750 + F::cast_from(0.3246312408709453_f64) * t17544;
    (t20748, t20753)
}
