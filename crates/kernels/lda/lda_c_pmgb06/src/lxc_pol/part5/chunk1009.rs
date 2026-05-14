//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1009/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1009<F: Float>(t13707: F, t20843: F, t20845: F, t20847: F, t20849: F, t20852: F, t20854: F, t20856: F, t20858: F, t20859: F, t20860: F, t1444: F, t7640: F, t1450: F, t493: F, t7639: F) -> (F, F, F) {
    let t20861 = -t20843 - t20845 - t20847 - t20849 + t13707 + t20852 + t20854 + t20856 + t20858 + t20859 - t20860;
    let t20863 = t1444 * t7640 / 45.0;
    let t20866 = t493 * t1450 * t7639 / 45.0;
    (t20861, t20863, t20866)
}
