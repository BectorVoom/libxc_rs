//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1311/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1311<F: Float>(t50970: F, t50972: F, t52159: F, t52167: F, t53038: F, t53053: F, t53058: F, t53065: F, t53072: F, t53126: F, t54942: F, t54946: F, t54952: F, t54957: F, t54962: F, t6793: F) -> F {
    let t54969 = -t54942 + t53038 / F::new(96.0) + t53053 / F::new(384.0) + t53058 / F::new(192.0) - t54946 - t53065 / F::new(384.0) + F::new(7.0) / F::new(36.0) * t50970 - F::new(7.0) / F::new(1152.0) * t50972 + t6793 * t54952 / F::new(24.0) + t6793 * t54957 / F::new(24.0) + t6793 * t54962 / F::new(24.0) + t53072 / F::new(96.0) + F::new(35.0) / F::new(108.0) * t52159 - F::new(7.0) / F::new(72.0) * t52167 - t53126 / F::new(12.0);
    t54969
}
