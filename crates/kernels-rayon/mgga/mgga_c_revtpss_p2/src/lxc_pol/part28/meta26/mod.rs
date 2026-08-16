//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta26 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk182;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk183;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk184;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk185;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk186;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta26(t117: f64, t93: f64, t19: f64, t22: f64, t30: f64, t153: f64, t33: f64, zeta_threshold: f64, t162: f64, t189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t511 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk182(t117, t93);
        let t512 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk183(t19, t22);
        let t513 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk184(t30);
        let (t514, t515, t516) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk185(t30, t513, t153, t33, zeta_threshold);
        let (t517, t520) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk186(t33, t516, t153, t515, t162, zeta_threshold);
        let t521 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk187(t189, t520);
    (t511, t512, t513, t514, t516, t517, t520, t521)
}
