//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1253/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1253<F: Float>(t53424: F, t3959: F, t8766: F, t13889: F, t14397: F, t14420: F, t19895: F, t2384: F, t2388: F, t2408: F, t3066: F, t3068: F, t35566: F, t4002: F, t4052: F, t51122: F, t51126: F, t51142: F, t53395: F, t53405: F, t53407: F, t53419: F, t6126: F, t6793: F, t8634: F, t9283: F) -> F {
    let t53425 = F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t53424;
    let t53426 = t3959 * t8766;
    let t53429 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51122 - t53395 / F::cast_from(768.0_f64) - t2384 * t14397 / F::cast_from(96.0_f64) - t8634 * t4002 / F::cast_from(48.0_f64) - t2388 * t14397 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51126 + t53405 + t53407 - t3066 * t9283 * t6126 * t4052 * t3068 / F::cast_from(8.0_f64) - t2408 * t35566 * t13889 / F::cast_from(12.0_f64) + t19895 * t14420 / F::cast_from(48.0_f64) + t6793 * t53419 / F::cast_from(24.0_f64) - t53425 + t53426 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51142;
    t53429
}
