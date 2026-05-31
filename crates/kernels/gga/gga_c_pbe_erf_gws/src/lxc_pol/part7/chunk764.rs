//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 764/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk764<F: Float>(t2170: F, t2171: F, t6269: F, t2168: F, t2251: F, t933: F, t2250: F) -> (F, F, F, F) {
    let t6271 = t2170 * t6269 * t2171;
    let t6273 = t2168 * t6271 / F::cast_from(16.0_f64);
    let t6274 = t2251 * t933;
    let t6275 = t2250 * t6274;
    (t6271, t6273, t6274, t6275)
}
