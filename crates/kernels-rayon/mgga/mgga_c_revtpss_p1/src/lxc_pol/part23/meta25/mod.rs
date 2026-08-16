//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta25 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk193;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk194;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk195;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk196;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk197;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk198;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta25(t460: f64, t490: f64, t488: f64, t198: f64, t336: f64, t424: f64, t452: f64, t454: f64, t265: f64, t33: f64, t57: f64, t398: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t117: f64, t93: f64, t19: f64, t22: f64, t30: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t493, t494) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk193(t460, t490);
        let t495 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk194(t488, t494);
        let (t498, t504, t502) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk195(t460, t495, t198, t336, t424, t452, t454, t265);
        let t508 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk196(t33, t265, t504, t57, t398, dens_threshold, rho1, zeta_threshold);
        let t511 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk197(t117, t93);
        let t512 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk198(t19, t22);
        let t513 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk199(t30);
    (t493, t494, t495, t498, t504, t502, t508, t511, t512, t513)
}
