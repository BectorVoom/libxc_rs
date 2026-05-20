//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta558<F: Float>(t7049: F, t786: F, t867: F, t2467: F, t2772: F, t689: F, t7014: F, t25338: F, t887: F, t2439: F, t25334: F, t7036: F, t820: F, t844: F, t2751: F, t2482: F, t814: F, t10782: F, t10744: F, t2664: F, t7028: F, t25240: F, t2693: F, t2710: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92921, t92922, t92925, t92930, t92935, t92951) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1999::<F>(t7049, t786, t867, t2467, t2772, t689, t7014, t25338, t887, t2439, t25334, t7036, t820, t844);
        let (t92952, t92955, t92956, t92963, t92966) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2000::<F>(t2751, t92951, t2482, t7036, t814, t10782, t10744, t2664, t7028, t25240, t2693, t2710);
    (t92921, t92922, t92925, t92930, t92935, t92951, t92952, t92955, t92956, t92963, t92966)
}
