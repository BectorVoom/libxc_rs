//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1407/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1407<F: Float>(t5571: F, t9323: F, t5635: F, t9586: F, t9425: F, t9318: F, t1857: F, t9342: F, t9855: F, t9410: F, t9413: F, t9372: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48269 = t5571 * t9323;
    let t48280 = t5635 * t9586;
    let t48282 = t5571 * t9425;
    let t48285 = t5571 * t9318;
    let t48287 = t9342 * t1857;
    let t48290 = t9855 * t1857;
    let t48292 = t9410 * t1857;
    let t48294 = t9413 * t1857;
    let t48297 = t5571 * t9372;
    (t48269, t48280, t48282, t48285, t48287, t48290, t48292, t48294, t48297)
}
