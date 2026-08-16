//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta570<F: Float>(t93282: F, t93317: F, t786: F, t860: F, t25410: F, t25413: F, t7064: F, t93150: F, t25375: F, t93311: F, t122: F, t7048: F, t72: F) -> (F, F, F, F, F, F, F) {
        let (t93318, t93320, t93321, t93322, t93324, t93326, t93329) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2019::<F>(t93282, t93317, t786, t860, t25410, t25413, t7064, t93150, t25375, t93311, t122, t7048, t72);
    (t93318, t93320, t93321, t93322, t93324, t93326, t93329)
}
