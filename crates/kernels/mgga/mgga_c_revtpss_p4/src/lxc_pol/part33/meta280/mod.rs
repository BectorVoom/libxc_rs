//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1250;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1251;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1252;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta280<F: Float>(t587: F, t65: F, t3140: F, t3736: F, t1276: F, t1243: F, t197: F, t532: F, t1450: F, t2033: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F, t121: F, t131: F, t141: F, t22: F, t2456: F, t624: F, t2501: F, t685: F, t793: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t8779, t8939, t8945) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1250::<F>(t587, t65, t3140, t3736, t1276, t1243);
        let (t8995, t8996, t9275, t9278) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1251::<F>(t197, t532, t1450, t2033, t143, t2580, t130, t2566, t700, t2584);
        let (t9283, t9285) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1252::<F>(t121, t131, t141, t22, t2456, t624);
        let (t9286, t9288) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1253::<F>(t2501, t9285, t685, t793);
    (t8779, t8939, t8945, t8995, t8996, t9275, t9278, t9283, t9285, t9286, t9288)
}
