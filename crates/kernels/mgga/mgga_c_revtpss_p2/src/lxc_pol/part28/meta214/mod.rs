//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1010;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1011;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1012;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1013;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1014;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1015;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta214<F: Float>(t1096: F, t1651: F, t1079: F, t2848: F, t3070: F, t4571: F, t4576: F, t4581: F, t4585: F, t996: F, t1678: F, t994: F, t1668: F, t73: F, t3095: F, t3092: F, t3093: F, t357: F, t1592: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4763, t4764) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1010::<F>(t1096, t1651, t1079);
        let t4772 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1011::<F>(t2848, t3070, t4571, t4576, t4581, t4585);
        let t4773 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1012::<F>(t4772, t996);
        let (t4778, t4781) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1013::<F>(t1678, t994, t1668, t73);
        let (t4782, t4783) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1014::<F>(t3095, t4781, t3092);
        let t4786 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1015::<F>(t3093, t357);
        let (t4787, t4788) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1016::<F>(t1592, t4786, t3092);
    (t4763, t4764, t4772, t4773, t4778, t4781, t4782, t4783, t4786, t4787, t4788)
}
