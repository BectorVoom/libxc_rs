//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk963;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk964;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk965;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk966;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk967;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk968;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk969;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta258<F: Float>(t2195: F, t625: F, t104: F, t109: F, t665: F, t108: F, t114: F, t661: F, t8258: F, t8267: F, t508: F, t569: F, t1453: F, t2198: F, t1312: F, t2199: F, t2201: F, t2322: F, t4254: F, t5523: F, t651: F, t8307: F, t3: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8310, t8311) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk963::<F>(t2195, t625, t104, t109);
        let (t8312, t8315) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk964::<F>(t665, t8311, t104, t108);
        let (t8316, t8320) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk965::<F>(t114, t661, t8315, t8258, t8267, t8310, t8312);
        let t8321 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk966::<F>(t508, t8320);
        let t8325 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk967::<F>(t569, t8320);
        let t8327 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk968::<F>(t1453, t2198);
        let (t8330, t8331) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk969::<F>(t1312, t2199, t2201, t2322, t4254, t5523, t651, t8307, t8321, t8325, t8327, t3);
        let t8336 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk970::<F>(t8330, param_d);
    (t8310, t8311, t8312, t8315, t8316, t8320, t8321, t8325, t8327, t8330, t8331, t8336)
}
