//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 319/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk319<F: Float>(t944: F, t945: F, t338: F, t828: F, t448: F, t80: F) -> (F, F, F, F) {
    let t946 = t944 * t945;
    let t1185 = t828 * t338;
    let t1214 = t448 * t80;
    let t1215 = F::cast_from(1.0_f64) / t1214;
    (t946, t1185, t1214, t1215)
}
