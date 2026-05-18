//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1145/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1145<F: Float>(t1385: F, t2064: F, t2574: F, t439: F, t493: F, t5486: F, t6782: F, t1420: F, t7711: F, t2948: F, t7710: F, t17563: F) -> (F, F, F, F, F) {
    let t20759 = F::new(2.0) / F::new(15.0) * t439 * t1385 * t2574 * t2064;
    let t20762 = t493 * t5486 * t6782 / F::new(15.0);
    let t20764 = F::new(2.0) / F::new(15.0) * t1420 * t7711;
    let t20767 = F::new(2.0) / F::new(15.0) * t439 * t2948 * t7710;
    let t20768 = t17563 / F::new(45.0);
    (t20759, t20762, t20764, t20767, t20768)
}
