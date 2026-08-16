//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3100/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3100<F: Float>(t16088: F, t3046: F, t380: F, t16139: F, t3127: F, t3172: F, t1011: F, t1655: F, t2438: F, t1014: F, t4579: F, t697: F) -> (F, F, F, F) {
    let t54089 = t3046 * t380 * t16088;
    let t54099 = t3127 * t3172 * t16139;
    let t54118 = t1011 * t2438 * t1655;
    let t54122 = t1011 * t697 * t1014 * t4579;
    (t54089, t54099, t54118, t54122)
}
