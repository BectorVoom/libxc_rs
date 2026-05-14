//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1265/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1265<F: Float>(t122055: F, t24432: F, t6118: F, t123957: F, t123894: F, t27805: F, t122679: F, t24437: F, t24438: F, t24531: F, t24543: F, t30987: F, t31041: F, t375: F, t89: F, t109363: F, t5092: F, t6119: F, t729: F, t747: F) -> (F, F, F, F, F, F, F) {
    let t124267 = t6118 * t24432 * t122055;
    let t124270 = t6118 * t24432 * t123957;
    let t124273 = t27805 * t24432 * t123894;
    let t124277 = t24437 * t24438 * t24531 * t122679;
    let t124279 = t24543 * t30987;
    let t124282 = t89 * t375 * t31041;
    let t124287 = t109363 * t729 * t6119 * t5092 * t747;
    (t124267, t124270, t124273, t124277, t124279, t124282, t124287)
}
