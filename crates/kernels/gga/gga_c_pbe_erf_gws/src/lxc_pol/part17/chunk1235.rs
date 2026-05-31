//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1235/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1235<F: Float>(t14404: F, t20113: F, t50970: F, t50972: F, t51890: F, t53028: F, t53034: F, t53038: F, t53042: F, t53047: F, t53053: F, t53058: F, t53061: F, t53065: F, t53072: F, t53075: F, t6793: F, t8793: F) -> F {
    let t53078 = -t53028 - t8793 * t51890 / F::cast_from(16.0_f64) + t20113 * t14404 / F::cast_from(48.0_f64) + t6793 * t53034 / F::cast_from(24.0_f64) + t53038 / F::cast_from(192.0_f64) + t6793 * t53042 / F::cast_from(24.0_f64) + t6793 * t53047 / F::cast_from(24.0_f64) + t53053 / F::cast_from(768.0_f64) + t53058 / F::cast_from(384.0_f64) - t53061 - t53065 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t50970 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t50972 + t53072 / F::cast_from(192.0_f64) + t6793 * t53075 / F::cast_from(24.0_f64);
    t53078
}
