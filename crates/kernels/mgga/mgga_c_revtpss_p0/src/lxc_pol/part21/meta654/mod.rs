//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2441;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta654<F: Float>(t11727: F, t3106: F, t3223: F, t3230: F, t11817: F, t3224: F, t1024: F, t11961: F, t3042: F, t3056: F, t225: F, t11274: F, t12009: F, t11277: F, t11916: F, t11246: F, t11251: F, t3172: F, t11648: F, t3169: F, t1062: F, t11782: F, t10356: F, t11150: F, t357: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42338, t42340, t42346, t42355, t42358, t42359, t42369) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2441::<F>(t11727, t3106, t3223, t3230, t11817, t3224, t1024, t11961, t3042, t3056, t225, t11274, t12009);
        let (t42374, t42377, t42383, t42391, t42397) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2442::<F>(t11277, t11916, t11246, t11251, t3172, t11648, t3169, t1062, t11782, t10356, t11150, t357);
    (t42338, t42340, t42346, t42355, t42358, t42359, t42369, t42374, t42377, t42383, t42391, t42397)
}
