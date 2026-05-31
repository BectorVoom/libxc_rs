//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 954/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk954<F: Float>(t187: F, t190: F, t367: F, t16672: F, t16682: F, t16690: F, t16697: F, t16706: F, t16715: F, t16722: F, t16724: F, t16728: F, t16730: F) -> F {
    let t17678 = F::cast_from(0.10864197530864197531e0_f64) * t190 * t367 * t187;
    let t17689 = t17678 - F::cast_from(0.86380000000000000002e0_f64) * t16672 - F::cast_from(0.71983333333333333335e-1_f64) * t16682 + F::cast_from(0.8638e0_f64) * t16690 + F::cast_from(0.21595e0_f64) * t16697 + F::cast_from(0.28793333333333333333e0_f64) * t16706 + F::cast_from(0.4798888888888888889e0_f64) * t16715 + F::cast_from(0.19195555555555555555e0_f64) * t16722 - F::cast_from(0.19195555555555555555e0_f64) * t16724 + F::cast_from(0.14929876543209876543e0_f64) * t16728 - F::cast_from(0.95977777777777777776e-1_f64) * t16730;
    t17689
}
