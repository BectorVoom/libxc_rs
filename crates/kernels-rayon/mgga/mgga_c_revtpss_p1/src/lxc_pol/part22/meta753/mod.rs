//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2827;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta753(t1011: f64, t3254: f64, t697: f64, t225: f64, t42051: f64, t1053: f64, t11788: f64, t11817: f64, t3211: f64, t1025: f64, t1026: f64, t2434: f64, t371: f64, t3191: f64, t3201: f64, t1021: f64, t11970: f64, t11874: f64, t15688: f64, t3224: f64, t3042: f64, t3056: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42257, t42261, t42265, t42270, t42274) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2827(t1011, t3254, t697, t225, t42051, t1053, t11788, t11817, t3211, t1025, t1026, t2434, t371);
        let (t42324, t42326, t42328, t42346, t42358, t42359, t42360) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2828(t3191, t3201, t1021, t11970, t11874, t15688, t11817, t3224, t3042, t3056, t225, t366);
    (t42257, t42261, t42265, t42270, t42274, t42324, t42326, t42328, t42346, t42358, t42359, t42360)
}
