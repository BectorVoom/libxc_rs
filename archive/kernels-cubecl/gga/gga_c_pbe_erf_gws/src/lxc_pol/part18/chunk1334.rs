//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1334/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1334<F: Float>(t14535: F, t3113: F, t54199: F, t57028: F, t57031: F, t57036: F, t57038: F, t57040: F, t57042: F, t57044: F, t57046: F, t57048: F, t57050: F, t57052: F) -> F {
    let t57054 = t3113 * t14535;
    let t57056 = t57028 / F::cast_from(48.0_f64) - t57031 / F::cast_from(48.0_f64) + t57036 / F::cast_from(48.0_f64) - t57038 / F::cast_from(48.0_f64) - t57040 / F::cast_from(48.0_f64) - t57042 / F::cast_from(384.0_f64) - t54199 + t57044 / F::cast_from(8.0_f64) - t57046 / F::cast_from(48.0_f64) - t57048 / F::cast_from(96.0_f64) + t57050 / F::cast_from(192.0_f64) + t57052 / F::cast_from(128.0_f64) - t57054 / F::cast_from(24.0_f64);
    t57056
}
