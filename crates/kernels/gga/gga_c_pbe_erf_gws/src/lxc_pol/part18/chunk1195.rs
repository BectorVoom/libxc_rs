//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1195/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1195<F: Float>(t3781: F, t3950: F, t850: F, t833: F, t13989: F, t14770: F, t14779: F, t14999: F, t15332: F, t15335: F, t15338: F, t15343: F, t15346: F, t15348: F, t15353: F, t15358: F, t15362: F, t15367: F, t15372: F, t15374: F, t2408: F, t3066: F) -> (F, F) {
    let t15377 = t850 * t3781 * t3950;
    let t15378 = t15377 * t833;
    let t15380 = t14999 - t15332 / F::cast_from(24.0_f64) - t15335 / F::cast_from(48.0_f64) + t13989 - t2408 * t15338 / F::cast_from(12.0_f64) - t15343 / F::cast_from(96.0_f64) - t15346 / F::cast_from(48.0_f64) - t15348 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t14770 - t3066 * t15353 / F::cast_from(16.0_f64) - t15358 / F::cast_from(3072.0_f64) + t3066 * t15362 / F::cast_from(24.0_f64) - t15367 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14779 + t15372 / F::cast_from(1536.0_f64) + t15374 / F::cast_from(96.0_f64) + t15378 / F::cast_from(96.0_f64);
    (t15377, t15380)
}
