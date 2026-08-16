//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 730/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk730<F: Float>(t13015: F, t13018: F, t13026: F, t13028: F, t13036: F, t13040: F, t13044: F, t13047: F, t13050: F, t13849: F, t13852: F, t13855: F) -> F {
    let t14498 = -t13015 - t13018 + t13026 + t13028 + t13036 - t13040 + t13044 - t13047 + F::cast_from(0.38342925953920749676e0_f64) * t13849 - F::cast_from(0.38342925953920749676e0_f64) * t13852 + t13855 - t13050;
    t14498
}
