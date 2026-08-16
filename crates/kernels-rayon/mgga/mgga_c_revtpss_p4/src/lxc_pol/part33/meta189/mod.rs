//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta189 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk887;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk888;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk889;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk890;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk891;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk892;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta189(t1699: f64, t3336: f64, t1100: f64, t1102: f64, t198: f64, t336: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4716: f64, t4718: f64, t4721: f64, t4723: f64, t4727: f64, t4731: f64, t4736: f64, t5019: f64, t5023: f64, t30: f64, t265: f64, t393: f64, t4560: f64, t1106: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t395: f64, t4186: f64, t45: f64, t4568: f64, t605: f64, t606: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1716: f64, t689: f64, t3362: f64, t3360: f64, t128: f64, t3367: f64, t1120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5024, t5027) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk887(t1699, t3336, t1100, t1102, t198, t336, t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736, t5019, t5023);
        let (t5028, t5035) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk888(t30, t265, t393, t4560, t5027, t1106, t1468, t1469, t1587, t1704, t395, t4186, t45, t4568, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t5044 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk889(t1716, t689);
        let (t5046, t5047) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk890(t1469, t3362, t606);
        let (t5048, t5049) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk891(t3360, t5047, t128);
        let (t5051, t5052) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk892(t1469, t3367, t606);
        let (t5053, t5054) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk893(t1120, t5052, t128);
    (t5024, t5028, t5035, t5044, t5046, t5047, t5048, t5049, t5051, t5052, t5053, t5054)
}
