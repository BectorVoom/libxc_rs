//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 618/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk618<F: Float>(t874: F, t368: F, t2279: F, t2281: F, t2312: F, t875: F, t158: F) -> (F, F, F, F, F) {
    let t2315 = t874 * t874;
    let t2316 = t368 * t368;
    let t2317 = 1.0 / t2316;
    let t2320 = -0.571528e-1 * t2279 + 0.285764e-1 * t2281 + 0.285764e-1 * t2312 * t875 - 0.285764e-1 * t2315 * t2317;
    let t2321 = t2320 * t158;
    (t2315, t2316, t2317, t2320, t2321)
}
