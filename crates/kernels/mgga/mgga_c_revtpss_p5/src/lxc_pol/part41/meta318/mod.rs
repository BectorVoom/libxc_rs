//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1093;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1094;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta318<F: Float>(t5697: F, t9962: F, t5701: F, t5608: F, t5675: F, t9934: F, t2661: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F, t124: F, t1882: F, t5609: F, t9794: F, t9793: F, t221: F, t5627: F, t9921: F, t3978: F, t2619: F, t5635: F, t1398: F, t3938: F, t9818: F, t9816: F, t125: F, t5658: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13810, t13813, t13832, t13845, t13846) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1093::<F>(t5697, t9962, t5701, t5608, t5675, t9934, t2661, t2482, t4000, t814, t136, t550);
        let (t13847, t13848, t13851, t13858, t13878) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1094::<F>(t13846, t220, t124, t1882, t5675, t13845, t5609, t9794, t9793, t221, t5627, t9921);
        let (t13880, t13887, t13926, t13943, t13944) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1095::<F>(t13878, t3978, t2619, t5635, t1398, t1882, t13848, t3938, t9818, t9816, t125, t5658);
    (t13810, t13813, t13832, t13847, t13848, t13851, t13858, t13880, t13887, t13926, t13943, t13944)
}
