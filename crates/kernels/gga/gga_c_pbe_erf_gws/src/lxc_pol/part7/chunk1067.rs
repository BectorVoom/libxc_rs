//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1067/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1067<F: Float>(t19234: F, t5761: F, t127: F, t1504: F, t1533: F, t19216: F, t19219: F, t19229: F, t19232: F, t19236: F, t19240: F, t19242: F, t19249: F, t19254: F, t5645: F, t5825: F, t5837: F) -> F {
    let t19256 = t5761 * t19234;
    let t19258 = -t19216 + t19219 - F::cast_from(0.1762848e3_f64) * t127 * t5825 * t1504 * t1533 + F::cast_from(0.2350464e2_f64) * t127 * t5837 * t5645 + t19229 - t19232 - t19236 - t19240 + F::cast_from(4.0_f64) * t19242 - t19249 + F::cast_from(0.1175232e2_f64) * t19254 + F::cast_from(0.783488e1_f64) * t19256;
    t19258
}
