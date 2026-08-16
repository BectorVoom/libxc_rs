//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk868;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk869;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk870;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta168(t3584: f64, t482: f64, t371: f64, t372: f64, t225: f64, t3555: f64, t480: f64, t3566: f64, t3568: f64, t1236: f64, t127: f64, t1235: f64, t221: f64, t462: f64, t696: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3661, t3663, t3666) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk868(t3584, t482, t371, t372, t225, t3555);
        let t3667 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk869(t3666, t480);
        let t3670 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk870(t225, t3566);
        let (t3671, t3672, t3674, t3678, t3679, t3682) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk871(t3670, t480, t3568, t482, t371, t372, t1236, t127, t1235, t221, t462, t696);
    (t3661, t3663, t3666, t3667, t3670, t3671, t3672, t3674, t3678, t3679, t3682)
}
