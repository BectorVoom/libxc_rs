//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1201/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1201<F: Float>(t3787: F, t519: F, t7003: F, t4753: F, t6993: F, t3416: F, t581: F, t6865: F, t1318: F, t1466: F, t593: F, t1325: F, t1440: F, t2166: F, t5127: F, t1476: F, t6205: F) -> (F, F, F, F, F, F) {
    let t17753 = t519 * t3787 * t7003;
    let t17754 = 16.0 / 45.0 * t17753;
    let t17756 = 8.0 / 15.0 * t4753 * t6993;
    let t17758 = 8.0 / 15.0 * t3416 * t6993;
    let t17759 = t581 * t6865;
    let t17763 = 8.0 / 15.0 * t1318 * t1466 * t17759 * t593;
    let t17767 = 8.0 / 15.0 * t1325 * t1440 * t2166 * t5127;
    let t17768 = t6205 * t1476;
    (t17754, t17756, t17758, t17763, t17767, t17768)
}
