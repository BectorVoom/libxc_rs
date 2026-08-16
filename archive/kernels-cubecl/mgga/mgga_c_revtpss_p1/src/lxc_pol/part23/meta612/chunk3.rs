//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2281/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2281<F: Float>(t1188: F, t24407: F, t12555: F, t24375: F, t1756: F, t20671: F, t1745: F, t6502: F, t1744: F, t20618: F, t1757: F, t6534: F) -> (F, F, F, F, F, F) {
    let t24408 = t24407 * t1188;
    let t24411 = t24375 * t12555;
    let t24414 = t20671 * t1756;
    let t24417 = t1745 * t6502;
    let t24420 = t20618 * t1744;
    let t24423 = t1757 * t6534;
    (t24408, t24411, t24414, t24417, t24420, t24423)
}
