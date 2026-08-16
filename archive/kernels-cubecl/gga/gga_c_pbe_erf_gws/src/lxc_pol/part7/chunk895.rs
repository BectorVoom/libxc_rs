//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 895/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk895<F: Float>(t395: F, t5090: F, t5399: F, t56: F, t1641: F, t1413: F) -> (F, F, F, F) {
    let t16968 = t395 * t5090;
    let t16970 = t56 * t5399;
    let t16971 = t1641 * t1641;
    let t16972 = F::cast_from(1.0_f64) / t16971;
    let t16973 = t1413 * t1413;
    (t16968, t16970, t16972, t16973)
}
