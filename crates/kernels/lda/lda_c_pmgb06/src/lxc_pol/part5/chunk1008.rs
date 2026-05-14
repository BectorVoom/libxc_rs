//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1008/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1008<F: Float>(t479: F, t7465: F, t2108: F, t2592: F, t486: F, t7443: F, t13182: F, t2469: F, t493: F, t2466: F, t5305: F, t1972: F, t6541: F, t6545: F, t17666: F, t17668: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20845 = t7465 * t479 / 30.0;
    let t20847 = t2592 * t2108 / 10.0;
    let t20849 = t486 * t7443 / 10.0;
    let t20852 = t493 * t13182 * t2469 / 9.0;
    let t20854 = t5305 * t2466 / 15.0;
    let t20856 = t1972 * t6541 / 15.0;
    let t20858 = t1972 * t6545 / 15.0;
    let t20859 = 4.0 / 15.0 * t17666;
    let t20860 = 2.0 / 5.0 * t17668;
    (t20845, t20847, t20849, t20852, t20854, t20856, t20858, t20859, t20860)
}
