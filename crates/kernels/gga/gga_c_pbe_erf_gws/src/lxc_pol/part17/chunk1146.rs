//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1146/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1146<F: Float>(t14538: F, t51282: F, t14535: F, t2129: F, t51306: F, t9500: F, t54183: F, t54186: F, t54188: F, t54190: F, t54192: F, t54194: F, t54196: F, t54199: F, t54201: F, t54203: F) -> (F,) {
    let t54205 = t14538 * t51282;
    let t54207 = t2129 * t14535;
    let t54209 = t51306 * t9500;
    let t54211 = t54183 / 96.0 + t54186 / 48.0 + t54188 / 24.0 + t54190 / 96.0 + t54192 / 128.0 + t54194 / 128.0 - t54196 / 32.0 - t54199 + t54201 / 96.0 - t54203 / 48.0 - t54205 / 96.0 - t54207 / 48.0 + t54209 / 48.0;
    (t54211,)
}
