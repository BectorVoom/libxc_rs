//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2827;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta753<F: Float>(t1011: F, t3254: F, t697: F, t225: F, t42051: F, t1053: F, t11788: F, t11817: F, t3211: F, t1025: F, t1026: F, t2434: F, t371: F, t3191: F, t3201: F, t1021: F, t11970: F, t11874: F, t15688: F, t3224: F, t3042: F, t3056: F, t366: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42257, t42261, t42265, t42270, t42274) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2827::<F>(t1011, t3254, t697, t225, t42051, t1053, t11788, t11817, t3211, t1025, t1026, t2434, t371);
        let (t42324, t42326, t42328, t42346, t42358, t42359, t42360) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2828::<F>(t3191, t3201, t1021, t11970, t11874, t15688, t11817, t3224, t3042, t3056, t225, t366);
    (t42257, t42261, t42265, t42270, t42274, t42324, t42326, t42328, t42346, t42358, t42359, t42360)
}
