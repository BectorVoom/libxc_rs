//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1134/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1134<F: Float>(t51222: F, t4023: F, t9179: F, t51215: F, t54019: F, t54021: F, t54024: F, t54026: F, t54027: F, t54029: F, t54031: F, t54033: F, t54035: F, t6645: F, t8991: F, t51351: F, t9612: F) -> (F, F, F) {
    let t54038 = 35.0 / 216.0 * t51222;
    let t54039 = t9179 * t4023;
    let t54041 = -t54019 / 96.0 - t54021 / 192.0 - t54024 / 24.0 + t54026 - t54027 / 24.0 - t54029 / 24.0 - t54031 / 192.0 + 5.0 / 96.0 * t54033 - t54035 / 128.0 + 7.0 / 1152.0 * t51215 + t54038 + t54039 / 48.0;
    let t54043 = t6645 * t8991;
    let t54045 = t51351 * t9612;
    (t54041, t54043, t54045)
}
