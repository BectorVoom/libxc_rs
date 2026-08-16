//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1578;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1579;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta536(t22857: f64, t550: f64, t2661: f64, t46609: f64, t9994: f64, t4003: f64, t9934: f64, t221: f64, t22809: f64, t3978: f64, t3979: f64, t22815: f64, t3989: f64, t22813: f64, t46716: f64, t1883: f64, t22020: f64, t3992: f64, t22877: f64, t46691: f64, t22822: f64, t543: f64, t22912: f64, t4018: f64, t4019: f64, t6869: f64, t73920: f64, t22245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86205, t86208, t86212, t86220, t86222) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1578(t22857, t550, t2661, t46609, t9994, t4003, t9934, t221, t22809, t3978, t3979, t22815, t3989);
        let (t86226, t86234, t86236, t86240) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1579(t221, t22813, t3978, t46716, t1883, t22020, t2661, t3992, t22877, t46691, t22822, t3989);
        let (t86244, t86256, t86260, t86264) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1580(t2661, t3992, t543, t86205, t221, t22912, t4018, t4019, t6869, t73920, t1883, t22245);
    (t86208, t86212, t86220, t86222, t86226, t86234, t86236, t86240, t86244, t86256, t86260, t86264)
}
