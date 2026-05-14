//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1163/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1163<F: Float>(t54166: F, t51256: F, t54158: F, t54160: F, t54162: F, t54164: F, t54168: F, t54170: F, t54173: F, t54175: F, t54177: F, t54179: F, t54198: F, t54183: F, t54186: F, t54188: F, t54190: F, t54192: F, t54194: F, t54196: F, t54201: F, t54203: F, t54205: F, t54207: F, t54209: F) -> (F, F) {
    let t55508 = 7.0 / 72.0 * t54166;
    let t55516 = -t54158 / 24.0 - t54160 / 12.0 - t54162 / 96.0 + t54164 / 48.0 + t55508 + t54168 / 12.0 + t54170 / 24.0 + 7.0 / 72.0 * t51256 - t54173 / 48.0 + 5.0 / 96.0 * t54175 + t54177 / 48.0 - t54179 / 32.0;
    let t55524 = 7.0 / 288.0 * t54198;
    let t55530 = t54183 / 48.0 + t54186 / 24.0 + t54188 / 12.0 + t54190 / 48.0 + t54192 / 64.0 + t54194 / 64.0 - t54196 / 16.0 - t55524 + t54201 / 48.0 - t54203 / 24.0 - t54205 / 48.0 - t54207 / 24.0 + t54209 / 24.0;
    (t55516, t55530)
}
