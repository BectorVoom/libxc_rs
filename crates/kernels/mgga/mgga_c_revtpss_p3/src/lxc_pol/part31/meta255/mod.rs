//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1129;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1130;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1131;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta255<F: Float>(t603: F, t607: F, t43: F, t48: F, t624: F, t49: F, t606: F, t613: F, t72: F, t1927: F, t640: F, t76: F, t1926: F, t5: F, t1923: F, t1928: F, t6954: F, t6958: F, t6960: F, t117: F, t116: F, t1931: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6963, t6968) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1129::<F>(t603, t607, t43, t48);
        let (t6971, t6972, t6973, t6974) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1130::<F>(t624, t49, t606, t613, t6968, t72, t1927);
        let (t6977, t6978) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1131::<F>(t640, t76, t1926);
        let (t6982, t6983, t6985) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1132::<F>(t5, t1923, t1928, t6954, t6958, t6960, t6963, t6974, t6978, t117, t116, t1931);
    (t6963, t6968, t6971, t6972, t6973, t6974, t6977, t6978, t6982, t6983, t6985)
}
