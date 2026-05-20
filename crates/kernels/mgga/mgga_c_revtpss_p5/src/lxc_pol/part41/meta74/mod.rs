//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk446;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk447;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk448;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk449;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta74<F: Float>(t1469: F, t36: F, t70: F, t48: F, t51: F, t53: F, rho1: F, sigma2: F, t60: F, t44: F, t56: F, t61: F, t626: F, t38: F, t633: F, t637: F, t77: F, t71: F, t85: F, t5: F, t1466: F, t603: F, t91: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1470, t1471, t1474, t1477, t1479, t1480) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk446::<F>(t1469, t36, t70, t48, t51, t53, rho1, sigma2);
        let (t1483, t1486) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk447::<F>(t1469, t60, t1474, t1480, t44, t56, t61, t626);
        let (t1487, t1494) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk448::<F>(t1486, t38, t1469, t633, t637, t77);
        let t1497 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk449::<F>(t1471, t1487, t1494, t71, t85);
        let t1501 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk450::<F>(t5, t1466, t1497, t603, t91);
    (t1470, t1471, t1474, t1477, t1479, t1480, t1483, t1486, t1487, t1494, t1497, t1501)
}
