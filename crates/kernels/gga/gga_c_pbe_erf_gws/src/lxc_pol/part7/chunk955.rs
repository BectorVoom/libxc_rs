//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 955/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk955<F: Float>(t127: F, t1504: F, t1533: F, t19216: F, t19219: F, t19229: F, t19232: F, t19236: F, t19240: F, t19242: F, t19249: F, t19254: F, t19256: F, t5645: F, t5825: F, t5837: F) -> (F,) {
    let t19258 = -t19216 + t19219 - 0.1762848e3 * t127 * t5825 * t1504 * t1533 + 0.2350464e2 * t127 * t5837 * t5645 + t19229 - t19232 - t19236 - t19240 + 4.0 * t19242 - t19249 + 0.1175232e2 * t19254 + 0.783488e1 * t19256;
    (t19258,)
}
