//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1207/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1207<F: Float>(t16941: F, t29361: F, t7908: F, t1380: F, t21827: F, t27370: F, t101994: F, t101997: F, t102001: F, t28353: F, t28369: F, t28372: F, t28373: F, t28420: F, t5732: F, t8155: F, t98025: F, t98138: F, t98150: F, t98155: F, t98162: F) -> (F, F) {
    let t103219 = t7908 * t16941 * t29361;
    let t103224 = t27370 * t21827 * t1380;
    let t103233 = -0.27802083333333333334e-2 * t7908 * t28372 * t28373 * t5732 - 0.46336805555555555556e-3 * t98025 * t8155 + 0.92673611111111111112e-3 * t28369 * t28420 - 0.10297067901234567901e-3 * t103219 - 0.18550940104166666667e-3 * t98138 + 0.30891203703703703704e-3 * t98150 + 0.13901041666666666667e-2 * t7908 * t103224 - 0.7369753086419753086e-3 * t98162 + 0.88437037037037037034e-2 * t101994 + 0.29479012345679012345e-2 * t101997 + 0.99491666666666666664e-2 * t102001 + 0.14840752083333333333e-2 * t98155 * t28353;
    (t103224, t103233)
}
