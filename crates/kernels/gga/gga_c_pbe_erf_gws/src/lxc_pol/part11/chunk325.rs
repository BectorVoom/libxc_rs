//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 325/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk325<F: Float>(t1218: F, t470: F, t155: F, t434: F, t433: F, t67: F, t62: F, t440: F) -> (F, F, F, F, F, F) {
    let t1219 = t470 * t1218;
    let t1220 = 0.11696446794910408142e1 * t1219;
    let t1224 = t155 * t434;
    let t1228 = t433 * t67;
    let t1229 = 1.0 / t1228;
    let t1230 = t62 * t1229;
    let t1231 = t440 * t440;
    (t1220, t1224, t1228, t1229, t1230, t1231)
}
