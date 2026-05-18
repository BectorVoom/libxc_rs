//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 982/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk982<F: Float>(t1503: F, t3685: F, t10033: F, t285: F, t545: F, t1473: F, t3626: F, t169: F, t301: F, t3373: F, t366: F, t159: F, t3379: F, t39: F) -> (F, F, F, F, F) {
    let t33637 = t1503 * t3685;
    let t33691 = t10033 * t545 * t285;
    let t33770 = t1473 * t3626;
    let t33778 = t169 * t366 * t3373 * t301;
    let t33837 = t39 * t3379 * t159 * t285;
    (t33637, t33691, t33770, t33778, t33837)
}
