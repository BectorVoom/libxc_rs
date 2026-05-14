//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1142/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1142<F: Float>(t14193: F, t22493: F, t53060: F, t14185: F, t3306: F, t353: F, t859: F, t1105: F, t4111: F, t4386: F, t1206: F, t2494: F, t50970: F, t50972: F, t52159: F, t52167: F, t53038: F, t53053: F, t53058: F, t53065: F, t53072: F, t53126: F, t6793: F) -> (F,) {
    let t54942 = 7.0 / 144.0 * t22493 * t14193;
    let t54946 = 7.0 / 288.0 * t53060;
    let t54952 = t859 * t353 * t14185 * t3306;
    let t54957 = t4386 * t353 * t4111 * t1105;
    let t54962 = t4386 * t353 * t1206 * t2494;
    let t54969 = -t54942 + t53038 / 96.0 + t53053 / 384.0 + t53058 / 192.0 - t54946 - t53065 / 384.0 + 7.0 / 36.0 * t50970 - 7.0 / 1152.0 * t50972 + t6793 * t54952 / 24.0 + t6793 * t54957 / 24.0 + t6793 * t54962 / 24.0 + t53072 / 96.0 + 35.0 / 108.0 * t52159 - 7.0 / 72.0 * t52167 - t53126 / 12.0;
    (t54969,)
}
