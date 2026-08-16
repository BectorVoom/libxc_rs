//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1862;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta532<F: Float>(t25386: F, t95536: F, t92840: F, t26518: F, t9285: F, t25299: F, t7407: F, t92890: F, t2061: F, t22: F, t25402: F, t93140: F, t25310: F, t26506: F, t26485: F, t93364: F, t2829: F, t689: F, t7384: F, t2439: F, t7398: F, t780: F, t785: F, t93134: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t95537, t95538, t95540, t95542, t95543, t95546, t95548) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1862::<F>(t25386, t95536, t92840, t26518, t9285, t25299, t7407, t92890, t2061, t22, t25402, t93140);
        let (t95551, t95553, t95556, t95562, t95567) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1863::<F>(t25310, t26506, t26485, t93364, t2829, t689, t7384, t2439, t7398, t780, t785, t93134, t95546);
    (t95537, t95538, t95540, t95542, t95543, t95548, t95551, t95553, t95556, t95562, t95567)
}
