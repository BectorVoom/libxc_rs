//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk861;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta178<F: Float>(t4772: F, t996: F, t1678: F, t994: F, t1668: F, t73: F, t3095: F, t3092: F, t3093: F, t357: F, t1592: F, t1058: F, t1660: F, t1053: F, t1659: F, t225: F, t4743: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4773, t4778, t4781) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk861::<F>(t4772, t996, t1678, t994, t1668, t73);
        let (t4782, t4783, t4786, t4787, t4788, t4792, t4794, t4797) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk862::<F>(t3095, t4781, t3092, t3093, t357, t1592, t1058, t1660, t1053, t1659, t225, t4743);
    (t4773, t4778, t4781, t4782, t4783, t4786, t4787, t4788, t4792, t4794, t4797)
}
