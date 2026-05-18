//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1243/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1243<F: Float>(t20581: F, t20584: F, t20587: F, t20589: F, t20592: F, t20596: F, t20601: F, t20602: F, t20603: F, t20604: F, t20608: F, t20610: F) -> F {
    let t22008 = -t20581 - t20584 + t20587 - t20589 - t20592 - t20596 - t20601 - t20602 - t20603 + t20604 - t20608 + t20610;
    t22008
}
