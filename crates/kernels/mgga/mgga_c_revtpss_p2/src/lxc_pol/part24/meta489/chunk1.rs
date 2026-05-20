//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1484/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1484<F: Float>(t124: F, t6843: F, t1412: F, t46766: F, t6864: F, t22267: F, t9976: F, t4010: F, t6816: F, t22027: F, t9775: F, t22263: F) -> (F, F, F, F, F, F, F) {
    let t73856 = t124 * t6843;
    let t73920 = t1412 * t6843;
    let t73929 = t46766 * t6864;
    let t73953 = t9976 * t22267;
    let t74012 = t4010 * t6816;
    let t74017 = t9775 * t22027;
    let t74024 = t9775 * t22263;
    (t73856, t73920, t73929, t73953, t74012, t74017, t74024)
}
