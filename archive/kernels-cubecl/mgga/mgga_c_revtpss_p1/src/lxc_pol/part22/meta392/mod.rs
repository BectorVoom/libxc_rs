//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1964;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1965;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta392<F: Float>(t1398: F, t1868: F, t3938: F, t13783: F, t3935: F, t828: F, t1882: F, t4003: F, t1353: F) -> (F, F, F, F, F, F) {
        let (t13784, t13785, t13786, t13789) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1964::<F>(t1398, t1868, t3938, t13783, t3935, t828);
        let t13790 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1965::<F>(t1882, t4003);
        let t13791 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1966::<F>(t1353, t1398);
    (t13784, t13785, t13786, t13789, t13790, t13791)
}
