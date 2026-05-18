//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 367/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk367<F: Float>(t1127: F, t1138: F, t1142: F, t1150: F, t1154: F, t1158: F, t882: F, t902: F, t914: F, t927: F, t929: F) -> F {
    let t1161 = t1127 - t1138 - t882 - t1142 + t902 * t1150 / F::new(1536.0) - t914 * t1154 / F::new(1536.0) - t927 - t929 * t1158 / F::new(768.0);
    t1161
}
