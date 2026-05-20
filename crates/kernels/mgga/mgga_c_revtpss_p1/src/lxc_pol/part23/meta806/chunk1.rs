//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2639/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2639<F: Float>(t231: F, t2782: F, t2783: F, t62868: F, t18729: F, t2470: F, t2798: F, t2723: F, t4503: F, t62760: F, t2482: F, t6016: F, t879: F) -> (F, F, F, F) {
    let t62938 = t2782 * t2783 * t62868 * t231;
    let t62952 = t2798 * t18729 * t2470;
    let t62961 = t2782 * t4503 * t62760 * t2723;
    let t62967 = t2482 * t879 * t6016;
    (t62938, t62952, t62961, t62967)
}
