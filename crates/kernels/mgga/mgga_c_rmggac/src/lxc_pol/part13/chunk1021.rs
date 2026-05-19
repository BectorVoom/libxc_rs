//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1021/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1021<F: Float>(t8494: F, t10276: F, t10277: F, t10278: F, t10279: F, t8073: F, t8074: F, t8075: F, t8076: F, t8077: F, t8080: F, t8498: F) -> (F, F) {
    let t42408 = F::cast_from(0.1702583995731913576e-4_f64) * t8494;
    let t42409 = -t8073 - t10276 + t10277 - t10278 + t10279 + t8074 - t8075 + t8076 - t8077 - t8080 - t42408;
    let t42413 = F::cast_from(0.1702583995731913576e-4_f64) * t8498;
    (t42409, t42413)
}
