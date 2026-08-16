//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta725 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2490;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta725(t46888: f64, t48908: f64, t1413: f64, t46835: f64, t48694: f64, t13775: f64, t9793: f64, t9794: f64, t5690: f64, t9741: f64, t2659: f64, t5744: f64, t816: f64, t10073: f64, t14124: f64, t5760: f64, t9292: f64, t10069: f64, t14207: f64, t40921: f64, t5737: f64, t225: f64, t2453: f64, t136: f64, t137: f64, t1398: f64, t14140: f64, t2438: f64, t4003: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49105, t49122, t49125, t49127, t49137) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2490(t46888, t48908, t1413, t46835, t48694, t13775, t9793, t9794, t5690, t9741, t2659, t5744, t816);
        let (t49167, t49172, t49177, t49178, t49180, t49186) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2491(t10073, t14124, t5760, t9292, t10069, t14207, t40921, t5737, t225, t2453, t136, t137, t1398, t14140, t2438, t4003);
    (t49105, t49122, t49125, t49127, t49137, t49167, t49172, t49177, t49178, t49180, t49186)
}
