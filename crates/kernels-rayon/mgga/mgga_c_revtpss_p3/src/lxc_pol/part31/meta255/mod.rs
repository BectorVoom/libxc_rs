//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1129;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1130;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1131;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta255(t603: f64, t607: f64, t43: f64, t48: f64, t624: f64, t49: f64, t606: f64, t613: f64, t72: f64, t1927: f64, t640: f64, t76: f64, t1926: f64, t5: f64, t1923: f64, t1928: f64, t6954: f64, t6958: f64, t6960: f64, t117: f64, t116: f64, t1931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6963, t6968) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1129(t603, t607, t43, t48);
        let (t6971, t6972, t6973, t6974) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1130(t624, t49, t606, t613, t6968, t72, t1927);
        let (t6977, t6978) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1131(t640, t76, t1926);
        let (t6982, t6983, t6985) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1132(t5, t1923, t1928, t6954, t6958, t6960, t6963, t6974, t6978, t117, t116, t1931);
    (t6963, t6968, t6971, t6972, t6973, t6974, t6977, t6978, t6982, t6983, t6985)
}
