//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 888/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk888<F: Float>(t1543: F, t537: F, t2124: F, t495: F, t2551: F, t5167: F, t2135: F, t2294: F, t2133: F, t120: F, t122: F, t135: F, t273: F, t57: F, t2096: F, t784: F) -> (F, F, F, F, F, F) {
    let t6294 = t537 * t1543;
    let t6296 = t2124 * t6294 * t495;
    let t6300 = t2124 * t5167 * t2551;
    let t6303 = t2294 * t2135;
    let t6304 = t2133 * t6303;
    let t6310 = 0.92480845007273388189e0 * t120 * t122 * t273 * t57 * t135;
    let t6311 = t2096 * t784;
    (t6296, t6300, t6303, t6304, t6310, t6311)
}
