//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1137;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta279<F: Float>(t126: F, t373: F, t828: F, t3119: F, t3115: F, t1086: F, t3057: F, t3090: F, t1043: F, t3059: F, t1045: F, t3117: F, t11671: F, t3114: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11921, t11922) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1137::<F>(t126, t373, t828);
        let (t11923, t11924, t11926, t11927, t11928, t11929, t11930, t11933) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1138::<F>(t11922, t3119, t3115, t1086, t3057, t3090, t1043, t3059, t1045, t3117, t11671, t3114);
    (t11921, t11922, t11923, t11924, t11926, t11927, t11928, t11929, t11930, t11933)
}
