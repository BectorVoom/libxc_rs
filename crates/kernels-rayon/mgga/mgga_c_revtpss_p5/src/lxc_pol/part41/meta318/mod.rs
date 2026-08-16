//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1093;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1094;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta318(t5697: f64, t9962: f64, t5701: f64, t5608: f64, t5675: f64, t9934: f64, t2661: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64, t220: f64, t124: f64, t1882: f64, t5609: f64, t9794: f64, t9793: f64, t221: f64, t5627: f64, t9921: f64, t3978: f64, t2619: f64, t5635: f64, t1398: f64, t3938: f64, t9818: f64, t9816: f64, t125: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13810, t13813, t13832, t13845, t13846) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1093(t5697, t9962, t5701, t5608, t5675, t9934, t2661, t2482, t4000, t814, t136, t550);
        let (t13847, t13848, t13851, t13858, t13878) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1094(t13846, t220, t124, t1882, t5675, t13845, t5609, t9794, t9793, t221, t5627, t9921);
        let (t13880, t13887, t13926, t13943, t13944) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1095(t13878, t3978, t2619, t5635, t1398, t1882, t13848, t3938, t9818, t9816, t125, t5658);
    (t13810, t13813, t13832, t13847, t13848, t13851, t13858, t13880, t13887, t13926, t13943, t13944)
}
