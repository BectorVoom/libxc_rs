//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta27 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk190;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk191;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk192;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk193;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk194;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk195;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk196;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk197;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk198;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta27(t530: f64, t136: f64, t221: f64, t149: f64, t225: f64, t522: f64, t524: f64, t73: f64, t235: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t531 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk190(t530);
        let t532 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk191(t530, t531);
        let t533 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk192(t531);
        let (t535, t539) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk193(t136, t221, t533, t149, t225, t522, t524);
        let t540 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk194(t532);
        let t541 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk195(t540, t73);
        let t543 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk196(t539, t541);
        let (t544, t545) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk197(t543);
        let t546 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk198(t225, t545);
        let t547 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk199(t235, t546);
    (t531, t532, t533, t535, t539, t540, t541, t543, t544, t545, t546, t547)
}
