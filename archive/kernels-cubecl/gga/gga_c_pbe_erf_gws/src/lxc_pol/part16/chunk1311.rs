//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1311/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1311<F: Float>(t50970: F, t50972: F, t52159: F, t52167: F, t53038: F, t53053: F, t53058: F, t53065: F, t53072: F, t53126: F, t54942: F, t54946: F, t54952: F, t54957: F, t54962: F, t6793: F) -> F {
    let t54969 = -t54942 + t53038 / F::cast_from(96.0_f64) + t53053 / F::cast_from(384.0_f64) + t53058 / F::cast_from(192.0_f64) - t54946 - t53065 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t50970 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t50972 + t6793 * t54952 / F::cast_from(24.0_f64) + t6793 * t54957 / F::cast_from(24.0_f64) + t6793 * t54962 / F::cast_from(24.0_f64) + t53072 / F::cast_from(96.0_f64) + F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t52159 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t52167 - t53126 / F::cast_from(12.0_f64);
    t54969
}
