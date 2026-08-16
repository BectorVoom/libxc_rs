//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta214 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1010;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1011;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1012;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1013;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1014;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1015;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta214(t1096: f64, t1651: f64, t1079: f64, t2848: f64, t3070: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t996: f64, t1678: f64, t994: f64, t1668: f64, t73: f64, t3095: f64, t3092: f64, t3093: f64, t357: f64, t1592: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4763, t4764) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1010(t1096, t1651, t1079);
        let t4772 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1011(t2848, t3070, t4571, t4576, t4581, t4585);
        let t4773 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1012(t4772, t996);
        let (t4778, t4781) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1013(t1678, t994, t1668, t73);
        let (t4782, t4783) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1014(t3095, t4781, t3092);
        let t4786 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1015(t3093, t357);
        let (t4787, t4788) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1016(t1592, t4786, t3092);
    (t4763, t4764, t4772, t4773, t4778, t4781, t4782, t4783, t4786, t4787, t4788)
}
