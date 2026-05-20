//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 550/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk550<F: Float>(t2724: F, t827: F, t828: F, t159: F, t243: F, t216: F, t124: F, t2394: F, t800: F, t2712: F, t785: F) -> (F, F, F, F, F, F) {
    let t2726 = t827 * t828 * t2724;
    let t2729 = t159 * t243;
    let t2730 = t216 * t2729;
    let t2731 = t124 * t2394;
    let t2732 = t800 * t2731;
    let t2735 = t2712 * t785;
    (t2726, t2729, t2730, t2731, t2732, t2735)
}
