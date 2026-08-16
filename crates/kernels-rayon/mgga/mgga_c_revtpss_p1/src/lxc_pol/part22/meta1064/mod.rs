//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1064 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3812;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1064(t48152: f64, t48154: f64, t3860: f64, t6801: f64, t3863: f64, t48158: f64, t46960: f64, t46964: f64, t46967: f64, t123: f64, t2630: f64, t6800: f64, t13656: f64, t198: f64, t46963: f64, t46970: f64, t6816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73327, t73328, t73330, t73332, t73333, t73334, t73338, t73339, t73341) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3812(t48152, t48154, t3860, t6801, t3863, t48158, t46960, t46964, t46967, t123, t2630, t6800);
        let (t73342, t73343) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3813(t73341, t13656, t198, t46963, t46970, t6816, t73327, t73328, t73330, t73332, t73333, t73334, t73338, t73339);
    (t73327, t73328, t73330, t73332, t73333, t73334, t73338, t73339, t73342, t73343)
}
