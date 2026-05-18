//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 720/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk720<F: Float>(t190: F, t4864: F, t8286: F, t147: F, t19: F, t457: F, t3156: F, t1458: F, t442: F, t567: F, t3116: F, t2937: F, t4026: F) -> (F, F, F, F, F, F, F) {
    let t8287 = t4864 * t190;
    let t8288 = t8286 * t8287;
    let t8290 = t457 * t19 * t147;
    let t8291 = t3156 * t8290;
    let t8292 = t8288 * t8291;
    let t8294 = t1458 * t190;
    let t8295 = t8286 * t8294;
    let t8296 = t442 * t567;
    let t8297 = t3116 * t8296;
    let t8298 = t8295 * t8297;
    let t8300 = t2937 * t4026;
    (t8290, t8291, t8292, t8296, t8297, t8298, t8300)
}
