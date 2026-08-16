//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1324/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1324<F: Float>(t12084: F, t4028: F, t11915: F, t4049: F, t11981: F, t54103: F, t54114: F, t54118: F, t56929: F, t56931: F, t56933: F, t56935: F, t56938: F, t56940: F, t56943: F) -> F {
    let t56945 = t4028 * t12084;
    let t56947 = t4049 * t11915;
    let t56949 = t4028 * t11981;
    let t56951 = t56929 / F::cast_from(96.0_f64) + t56931 / F::cast_from(96.0_f64) + t56933 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t56935 + t56938 / F::cast_from(16.0_f64) + t54103 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t56940 - t56943 / F::cast_from(12.0_f64) + t54114 + t54118 - t56945 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t56947 - t56949 / F::cast_from(48.0_f64);
    t56951
}
