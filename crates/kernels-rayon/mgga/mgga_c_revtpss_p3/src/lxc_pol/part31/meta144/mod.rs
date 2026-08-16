//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk763;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta144(t3781: f64, t460: f64, t3303: f64, t471: f64, t498: f64, t1330: f64, t72: f64, t757: f64, t530: f64, t566: f64, t525: f64, t527: f64, t2608: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3782 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk763(t3781, t460);
        let (t3783, t3800, t3801, t3825, t3826, t3828, t3833, t3841, t3853) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk764(t3303, t471, t498, t1330, t72, t757, t530, t566, t525, t527, t2608, t520);
    (t3782, t3783, t3800, t3801, t3825, t3826, t3828, t3833, t3841, t3853)
}
