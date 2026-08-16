//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1718;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1719;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta360<F: Float>(t11922: F, t3119: F, t3115: F, t1086: F, t3057: F, t3090: F, t1043: F, t3059: F, t1045: F, t3117: F, t11671: F, t3114: F, t127: F, t3206: F, t371: F, t3205: F, t11200: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11923, t11924, t11926, t11927) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1718::<F>(t11922, t3119, t3115, t1086, t3057, t3090);
        let (t11928, t11929, t11930, t11933) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1719::<F>(t1043, t3059, t1045, t3117, t11671, t3114);
        let (t11937, t11938, t11940) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1720::<F>(t127, t3206, t371, t3205, t11200, t225);
    (t11923, t11924, t11926, t11927, t11928, t11929, t11930, t11933, t11937, t11938, t11940)
}
