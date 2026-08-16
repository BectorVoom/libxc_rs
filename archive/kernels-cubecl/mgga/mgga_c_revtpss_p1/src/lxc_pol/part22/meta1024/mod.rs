//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1024 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3580;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3581;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3582;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3583;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3584;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1024<F: Float>(t20265: F, t2258: F, t128: F, t3360: F, t12268: F, t5825: F, t2251: F, t18281: F, t3367: F, t606: F, t1120: F, t20317: F, t43766: F, t5819: F, t43860: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t68285, t68287) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3580::<F>(t20265, t2258, t128, t3360);
        let (t68290, t68292) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3581::<F>(t12268, t5825, t2251, t128, t3360);
        let (t68295, t68297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3582::<F>(t18281, t3367, t606, t1120, t128);
        let (t68299, t68301) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3583::<F>(t20317, t2258, t1120, t128);
        let (t68303, t68305) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3584::<F>(t20265, t2251, t1120, t128);
        let (t68308, t68310) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3585::<F>(t2251, t43766, t5819, t128, t43860);
    (t68285, t68287, t68290, t68292, t68295, t68297, t68299, t68301, t68303, t68305, t68308, t68310)
}
