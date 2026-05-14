//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1044/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1044<F: Float>(t519: F, t7683: F, t9304: F, t17718: F, t17753: F, t17768: F, t12314: F, t6753: F, t16602: F, t1949: F, t4506: F, t1944: F, t2526: F, t4521: F, t6743: F, t6749: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21737 = t519 * t9304 * t7683;
    let t21738 = 16.0 / 45.0 * t21737;
    let t21739 = 32.0 / 135.0 * t17718;
    let t21740 = 8.0 / 15.0 * t17753;
    let t21741 = 8.0 / 45.0 * t17768;
    let t21743 = 16.0 / 9.0 * t12314 * t6753;
    let t21746 = 8.0 / 15.0 * t4506 * t16602 * t1949;
    let t21750 = 4.0 / 9.0 * t4506 * t4521 * t2526 * t1944;
    let t21752 = 16.0 / 15.0 * t12314 * t6743;
    let t21754 = 32.0 / 15.0 * t12314 * t6749;
    (t21738, t21739, t21740, t21741, t21743, t21746, t21750, t21752, t21754)
}
