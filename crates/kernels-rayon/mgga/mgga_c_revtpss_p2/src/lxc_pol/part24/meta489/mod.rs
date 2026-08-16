//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1483;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta489(t22449: f64, t2435: f64, t136: f64, t2457: f64, t6918: f64, t9674: f64, t124: f64, t6861: f64, t46917: f64, t6871: f64, t22102: f64, t46740: f64, t6843: f64, t1412: f64, t46766: f64, t6864: f64, t22267: f64, t9976: f64, t4010: f64, t6816: f64, t22027: f64, t9775: f64, t22263: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73707, t73712, t73731, t73778, t73789) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1483(t22449, t2435, t136, t2457, t6918, t9674, t124, t6861, t46917, t6871, t22102, t46740);
        let (t73856, t73920, t73929, t73953, t74012, t74017, t74024) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1484(t124, t6843, t1412, t46766, t6864, t22267, t9976, t4010, t6816, t22027, t9775, t22263);
    (t73707, t73712, t73731, t73778, t73789, t73856, t73920, t73929, t73953, t74012, t74017, t74024)
}
