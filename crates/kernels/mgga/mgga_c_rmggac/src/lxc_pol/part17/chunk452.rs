//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 452/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk452<F: Float>(t433: F, t5969: F, t498: F, t5389: F, t4085: F, t4112: F, t4114: F, t1425: F, t4056: F, t4062: F, t4064: F, t4074: F, t4077: F, t4080: F, t4083: F, t4089: F, t4101: F, t4106: F, t4111: F, t5375: F, t5376: F, t5377: F, t5382: F, t5395: F) -> (F, F, F, F, F) {
    let t5970 = t5969 * t433;
    let t5971 = 0.5848223622634646207e0 * t5970;
    let t5974 = t5389 * t498;
    let t5977 = 0.10843581300301739842e-1 * t4085;
    let t5978 = 32.0 * t4112;
    let t5979 = 20.0 * t4114;
    let t5980 = -t4056 + t4062 + t4064 + t5375 - t5376 - t4074 - t5971 + 0.373092e0 * t5395 * t5377 - 0.186546e0 * t1425 * t5974 - t4077 - t4080 + t4083 + t5977 + t4089 - t4101 + t4106 + t4111 + t5978 + t5979 - t5382;
    (t5971, t5977, t5978, t5979, t5980)
}
