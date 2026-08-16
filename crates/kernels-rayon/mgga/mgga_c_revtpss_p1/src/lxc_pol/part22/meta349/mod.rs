//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1837;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1838;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1839;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta349(t11773: f64, t3114: f64, t1024: f64, t3230: f64, t11213: f64, t225: f64, t366: f64, t11223: f64, t1053: f64, t3223: f64, t3215: f64, t3224: f64, t3111: f64, t3188: f64, t3211: f64, t1026: f64, t371: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t11774 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1837(t11773, t3114);
        let (t11779, t11782) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1838(t1024, t3230, t11213, t225);
        let (t11783, t11788) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1839(t11782, t366, t11223, t225);
        let (t11789, t11792, t11795, t11802, t11814, t11817) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1840(t11788, t366, t1053, t3223, t3215, t3224, t3111, t3188, t3211, t1026, t371, t676);
    (t11774, t11779, t11782, t11783, t11788, t11789, t11792, t11795, t11802, t11814, t11817)
}
