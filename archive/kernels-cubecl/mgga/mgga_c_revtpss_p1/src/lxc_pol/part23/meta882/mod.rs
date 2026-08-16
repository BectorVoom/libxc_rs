//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta882 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2792;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta882<F: Float>(t10090: F, t122: F, t14144: F, t2482: F, t6861: F, t72: F, t9994: F, t14145: F, t4114: F, t10014: F, t22336: F, t1398: F, t73820: F, t2782: F, t47371: F, t6862: F, t10022: F, t22315: F, t46457: F, t136: F, t2457: F, t47429: F, t22332: F) -> (F, F, F, F, F, F, F, F) {
        let (t75035, t75039, t75041, t75047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2792::<F>(t10090, t122, t14144, t2482, t6861, t72, t9994, t14145, t4114, t10014, t22336, t1398, t73820);
        let (t75049, t75053, t75060, t75068, t75071) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2793::<F>(t2782, t47371, t75047, t1398, t6862, t10022, t22315, t46457, t136, t2457, t47429, t10014, t22332);
    (t75035, t75039, t75041, t75049, t75053, t75060, t75068, t75071)
}
