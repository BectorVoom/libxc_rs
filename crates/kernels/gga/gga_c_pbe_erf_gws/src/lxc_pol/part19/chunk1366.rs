//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1366/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1366<F: Float>(t1206: F, t353: F, t3703: F, t8599: F, t11354: F, t14881: F, t15537: F, t22343: F, t3066: F, t55248: F, t55251: F, t55258: F, t56593: F, t56596: F, t56599: F, t56604: F, t56613: F, t56618: F, t56626: F, t56638: F, t56642: F, t6793: F, t9283: F) -> F {
    let t58264 = t8599 * t353 * t1206 * t3703;
    let t58280 = t22343 * t15537 / F::cast_from(96.0_f64) - t6793 * t58264 / F::cast_from(16.0_f64) + t56593 / F::cast_from(24.0_f64) - t55248 + t56596 / F::cast_from(768.0_f64) - t55251 - t55258 + t56599 / F::cast_from(48.0_f64) + t56604 / F::cast_from(192.0_f64) - t3066 * t9283 * t14881 * t11354 / F::cast_from(16.0_f64) - t56613 / F::cast_from(768.0_f64) + t56618 / F::cast_from(384.0_f64) - t56626 / F::cast_from(48.0_f64) - t56638 / F::cast_from(384.0_f64) - t56642 / F::cast_from(768.0_f64);
    t58280
}
