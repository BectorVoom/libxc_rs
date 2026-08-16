//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta17 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk122;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk123;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk124;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk125;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk126;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta17(t273: f64, t276: f64, t279: f64, t285: f64, t293: f64, t300: f64, t302: f64, t199: f64, t240: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t307, t310, t311) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk122(t273, t276, t279, t285);
        let t315 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk123(t273);
        let (t320, t323, t324) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk124(t273, t276, t279, t285);
        let (t328, t330, t334, t335) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk125(t315, t324, t293, t300, t302, t311, t199, t240, zeta_threshold);
        let t336 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk126(t334, t335);
    (t307, t310, t311, t315, t320, t323, t324, t328, t330, t334, t335, t336)
}
