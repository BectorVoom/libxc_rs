//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 767/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk767<F: Float>(t24477: F, t2574: F, t6119: F, t24437: F, t1439: F, t1636: F, t89: F, t375: F, t6144: F, t24395: F, t676: F, t27: F, t24191: F, t713: F, t193: F, t2459: F, t6008: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24479 = t2574 * t6119 * t24477;
    let t24480 = t24437 * t24479;
    let t24482 = t89 * t1636 * t1439;
    let t24483 = 4.0 / 9.0 * t24482;
    let t24485 = t89 * t375 * t6144;
    let t24487 = t676 * t24395;
    let t24489 = t89 * t27 * t24487;
    let t24490 = t24191 * t713;
    let t24491 = t193 * t24490;
    let t24492 = t89 * t24491;
    let t24494 = t6008 * t2459;
    (t24479, t24480, t24482, t24483, t24485, t24487, t24489, t24490, t24492, t24494)
}
