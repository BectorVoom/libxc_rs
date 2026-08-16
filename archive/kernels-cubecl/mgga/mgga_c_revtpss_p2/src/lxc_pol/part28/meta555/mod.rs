//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2007;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta555<F: Float>(t25338: F, t689: F, t887: F, t2439: F, t25334: F, t7036: F, t820: F, t844: F, t2751: F, t2482: F, t814: F, t10782: F, t10744: F, t2664: F, t7028: F, t25240: F, t2693: F, t2710: F, t228: F, t25273: F, t802: F, t25277: F, t2707: F, t25282: F, t9802: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92930, t92935, t92951, t92952, t92955, t92956) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2007::<F>(t25338, t689, t887, t2439, t25334, t7036, t820, t844, t2751, t2482, t814, t10782);
        let (t92963, t92966, t92968, t92969, t92971, t92975) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2008::<F>(t10744, t2664, t7028, t25240, t2693, t2710, t228, t25273, t802, t25277, t2707, t25282, t9802);
    (t92930, t92935, t92951, t92952, t92955, t92956, t92963, t92966, t92968, t92969, t92971, t92975)
}
