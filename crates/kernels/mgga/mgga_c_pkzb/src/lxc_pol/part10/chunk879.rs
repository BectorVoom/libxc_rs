//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 879/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk879<F: Float>(t218: F, t2226: F, t675: F, t2230: F, t2238: F, t831: F, t338: F) -> (F, F, F, F) {
    let t6180 = t218 * t675 * t2226;
    let t6183 = t218 * t675 * t2230;
    let t6198 = 1.0 / t2238 / t831;
    let t6199 = t338 * t6198;
    (t6180, t6183, t6198, t6199)
}
