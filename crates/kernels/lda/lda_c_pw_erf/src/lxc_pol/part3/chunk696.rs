//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 696/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk696<F: Float>(t1976: F, t494: F, t4829: F, t1325: F, t2030: F, t3802: F, t519: F, t1381: F, t816: F, t1308: F, t571: F, t2151: F, t581: F, t1954: F, t593: F, t1472: F, t2014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4830 = t1976 * t494;
    let t4831 = t4829 * t4830;
    let t4833 = 32.0 / 45.0 * t1325 * t4831;
    let t4834 = t3802 * t2030;
    let t4836 = 16.0 / 135.0 * t519 * t4834;
    let t4837 = t816 * t1381;
    let t4838 = t1308 * t4837;
    let t4840 = 4.0 / 45.0 * t571 * t4838;
    let t4841 = t2151 * t581;
    let t4842 = t1954 * t593;
    let t4843 = t4841 * t4842;
    let t4845 = 16.0 / 45.0 * t571 * t4843;
    let t4847 = 16.0 / 45.0 * t1472 * t2014;
    (t4830, t4831, t4833, t4834, t4836, t4837, t4838, t4840, t4841, t4842, t4843, t4845, t4847)
}
