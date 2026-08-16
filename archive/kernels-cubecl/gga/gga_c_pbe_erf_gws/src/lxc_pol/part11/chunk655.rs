//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 655/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk655<F: Float>(t2264: F, t899: F, t922: F, t2331: F, t900: F, t3205: F, t336: F, t2153: F, t837: F, t863: F, t2262: F, t344: F) -> (F, F, F, F, F) {
    let t6501 = t899 * t2264 * t922;
    let t6505 = t899 * t900 * t2331;
    let t6523 = t3205 * t336;
    let t6542 = t863 * t2153 * t837;
    let t6552 = F::cast_from(1.0_f64) / t2262 / t344;
    (t6501, t6505, t6523, t6542, t6552)
}
