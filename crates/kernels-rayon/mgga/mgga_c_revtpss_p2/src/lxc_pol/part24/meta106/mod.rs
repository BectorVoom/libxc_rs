//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk607;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk608;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk609;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk610;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta106(t1264: f64, t828: f64, t1121: f64, t471: f64, t126: f64, t1263: f64, t371: f64, t482: f64, t676: f64, t481: f64, t225: f64, t3566: f64, t480: f64, t221: f64, t462: f64, t696: f64, t461: f64, t1224: f64, t3367: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3626 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk607(t1264, t828);
        let (t3628, t3634) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk608(t1121, t471, t126, t1263);
        let t3655 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk609(t371, t482, t676);
        let (t3657, t3670) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk610(t3655, t481, t225, t3566);
        let (t3671, t3682, t3684, t3692, t3698) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk611(t3670, t480, t221, t462, t696, t461, t1224, t3367, t1121, t404);
    (t3626, t3628, t3634, t3655, t3657, t3670, t3671, t3682, t3684, t3692, t3698)
}
