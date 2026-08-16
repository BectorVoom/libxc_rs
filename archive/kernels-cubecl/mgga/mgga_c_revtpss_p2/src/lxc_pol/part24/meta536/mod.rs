//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1578;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1579;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta536<F: Float>(t22857: F, t550: F, t2661: F, t46609: F, t9994: F, t4003: F, t9934: F, t221: F, t22809: F, t3978: F, t3979: F, t22815: F, t3989: F, t22813: F, t46716: F, t1883: F, t22020: F, t3992: F, t22877: F, t46691: F, t22822: F, t543: F, t22912: F, t4018: F, t4019: F, t6869: F, t73920: F, t22245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86205, t86208, t86212, t86220, t86222) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1578::<F>(t22857, t550, t2661, t46609, t9994, t4003, t9934, t221, t22809, t3978, t3979, t22815, t3989);
        let (t86226, t86234, t86236, t86240) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1579::<F>(t221, t22813, t3978, t46716, t1883, t22020, t2661, t3992, t22877, t46691, t22822, t3989);
        let (t86244, t86256, t86260, t86264) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1580::<F>(t2661, t3992, t543, t86205, t221, t22912, t4018, t4019, t6869, t73920, t1883, t22245);
    (t86208, t86212, t86220, t86222, t86226, t86234, t86236, t86240, t86244, t86256, t86260, t86264)
}
