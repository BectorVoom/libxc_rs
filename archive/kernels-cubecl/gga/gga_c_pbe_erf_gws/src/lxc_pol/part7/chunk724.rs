//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 724/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk724<F: Float>(t2004: F, t5953: F, t5356: F, t5359: F, t5375: F, t5377: F, t5381: F, t5397: F, t5405: F, t5933: F, t5936: F, t5938: F, t5940: F, t5944: F, t5945: F, t5948: F, t5949: F, t5952: F) -> F {
    let t5954 = t5953 * t2004;
    let t5956 = t5933 + F::cast_from(0.32463124087094530131e0_f64) * t5936 + F::cast_from(0.64926248174189060262e0_f64) * t5938 + F::cast_from(0.21642082724729686754e0_f64) * t5940 - t5944 - t5356 + t5359 + t5375 + F::cast_from(8.0_f64) * t5945 + t5948 + F::cast_from(4.0_f64) * t5949 + t5952 - t5377 + t5381 + F::cast_from(0.33545228223331014468e-1_f64) * t5954 + t5397 + t5405;
    t5956
}
