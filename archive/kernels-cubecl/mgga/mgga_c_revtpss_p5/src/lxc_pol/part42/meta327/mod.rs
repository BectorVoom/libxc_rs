//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1115;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1116;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta327<F: Float>(t1469: F, t2609: F, t706: F, t1568: F, t785: F, t780: F, t2439: F, t212: F, t4469: F, t689: F, t1579: F, t2769: F, t886: F, t252: F, t2782: F, t2470: F, t4480: F, t2465: F, t1558: F, t836: F, t231: F, t2797: F, t860: F, t2783: F, t251: F, t4423: F, t10073: F, t4496: F, t10542: F, t4500: F, t4424: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14441, t14474, t14479, t14480) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1115::<F>(t1469, t2609, t706, t1568, t785, t780, t2439, t212, t4469, t689, t1579, t2769);
        let (t14484, t14486, t14494, t14498, t14502) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1116::<F>(t14480, t886, t252, t2782, t2470, t4480, t2465, t1558, t836, t231, t2797, t860);
        let (t14506, t14511, t14512, t14518, t14519) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1117::<F>(t14502, t231, t2783, t2782, t251, t4423, t10073, t4496, t10542, t4500, t4424, t72);
    (t14441, t14474, t14479, t14484, t14486, t14494, t14498, t14506, t14511, t14512, t14518, t14519)
}
