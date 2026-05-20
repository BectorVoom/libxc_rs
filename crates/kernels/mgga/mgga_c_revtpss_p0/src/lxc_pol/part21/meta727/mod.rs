//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta727 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2568;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta727<F: Float>(t2482: F, t4000: F, t596: F, t10003: F, t1412: F, t3923: F, t2661: F, t9835: F, t9934: F, t9914: F, t9918: F, t221: F, t4018: F, t4019: F, t9899: F, t4059: F, t9909: F, t9812: F, t9962: F, t13845: F, t46751: F, t9818: F, t13847: F, t9819: F, t9840: F, t9958: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47216, t47218, t47221, t47223, t47227) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2568::<F>(t2482, t4000, t596, t10003, t1412, t3923, t2661, t9835, t9934, t9914, t9918, t221, t4018, t4019, t9899);
        let (t47229, t47231, t47235, t47239, t47245) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2569::<F>(t4059, t9909, t9812, t9962, t13845, t46751, t9818, t9835, t13847, t9819, t9840, t9958);
    (t47216, t47218, t47221, t47223, t47227, t47229, t47231, t47235, t47239, t47245)
}
