//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk846;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk847;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk848;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta167(t3153: f64, t3603: f64, t3602: f64, t1042: f64, t1244: f64, t3598: f64, t3594: f64, t471: f64, t1121: f64, t414: f64, t66: f64, t3363: f64, t247: f64, t474: f64, t479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3604, t3605, t3606) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk846(t3153, t3603, t3602, t1042);
        let (t3609, t3610, t3611, t3612, t3613) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk847(t1244, t3598, t3594, t3153, t471, t3602, t1042);
        let t3617 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk848(t1121, t414);
        let (t3618, t3620, t3623) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk849(t3617, t66, t3363, t247, t474, t479);
    (t3604, t3605, t3606, t3609, t3610, t3611, t3612, t3613, t3617, t3618, t3620, t3623)
}
