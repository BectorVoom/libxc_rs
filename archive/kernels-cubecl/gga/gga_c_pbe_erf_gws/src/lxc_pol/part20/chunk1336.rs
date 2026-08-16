//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1336/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1336<F: Float>(t11689: F, t14007: F, t14535: F, t3108: F, t11953: F, t14015: F, t54237: F, t54239: F, t57060: F, t57062: F, t57064: F, t57066: F, t57068: F, t57070: F, t57073: F) -> F {
    let t57075 = t14007 * t11689;
    let t57077 = t3108 * t14535;
    let t57079 = t14015 * t11953;
    let t57081 = -t57060 / F::cast_from(24.0_f64) - t57062 / F::cast_from(192.0_f64) - t57064 / F::cast_from(48.0_f64) + t57066 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57068 + t57070 / F::cast_from(192.0_f64) + t54237 - t57073 / F::cast_from(96.0_f64) - t57075 / F::cast_from(192.0_f64) - t57077 / F::cast_from(24.0_f64) - t54239 - t57079 / F::cast_from(96.0_f64);
    t57081
}
