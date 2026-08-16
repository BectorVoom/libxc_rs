//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1483;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta489<F: Float>(t22449: F, t2435: F, t136: F, t2457: F, t6918: F, t9674: F, t124: F, t6861: F, t46917: F, t6871: F, t22102: F, t46740: F, t6843: F, t1412: F, t46766: F, t6864: F, t22267: F, t9976: F, t4010: F, t6816: F, t22027: F, t9775: F, t22263: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t73707, t73712, t73731, t73778, t73789) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1483::<F>(t22449, t2435, t136, t2457, t6918, t9674, t124, t6861, t46917, t6871, t22102, t46740);
        let (t73856, t73920, t73929, t73953, t74012, t74017, t74024) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1484::<F>(t124, t6843, t1412, t46766, t6864, t22267, t9976, t4010, t6816, t22027, t9775, t22263);
    (t73707, t73712, t73731, t73778, t73789, t73856, t73920, t73929, t73953, t74012, t74017, t74024)
}
