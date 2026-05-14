//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1191/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1191<F: Float>(t224: F, t6687: F, t500: F, t1451: F, t6134: F, t17990: F, t17992: F, t17994: F, t17997: F, t18001: F, t18003: F, t18005: F, t18007: F, t18009: F, t18011: F, t18013: F, t18015: F, t18019: F) -> (F, F, F) {
    let t18020 = t6687 * t224;
    let t18022 = 2.0 / 45.0 * t18020 * t500;
    let t18024 = 2.0 / 45.0 * t6134 * t1451;
    let t18025 = t17990 + t17992 + t17994 - t17997 - t18001 + t18003 + t18005 + t18007 - t18009 - t18011 + t18013 + t18015 + t18019 + t18022 + t18024;
    (t18022, t18024, t18025)
}
