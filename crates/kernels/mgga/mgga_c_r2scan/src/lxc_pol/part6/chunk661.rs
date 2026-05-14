//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 661/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk661<F: Float>(t2155: F, t2605: F, t113: F, t2572: F, t2148: F, t2147: F, t481: F, t938: F) -> (F, F, F, F, F) {
    let t2606 = t2155 * t2605;
    let t2608 = t2572 * t113;
    let t2609 = t2148 * t2608;
    let t2610 = t2147 * t2609;
    let t2612 = t938 * t481;
    (t2606, t2608, t2609, t2610, t2612)
}
