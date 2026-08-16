//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1363/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1363<F: Float>(t15292: F, t840: F, t361: F, t57321: F, t13917: F, t3223: F, t12014: F, t13919: F, t1115: F, t13911: F, t13930: F, t14397: F, t15273: F, t2498: F, t335: F, t338: F, t35057: F, t4002: F, t53617: F, t53939: F, t54488: F, t57402: F, t57404: F, t57410: F, t57415: F, t57422: F, t8629: F, t8793: F, t892: F, t9858: F) -> F {
    let t57428 = t840 * t15292;
    let t57432 = t361 * t57321;
    let t57434 = t13917 * t57432 * t3223;
    let t57441 = t13917 * t13919 * t12014;
    let t57445 = t57402 / F::cast_from(24.0_f64) + t57404 / F::cast_from(24.0_f64) - t2498 * t14397 / F::cast_from(48.0_f64) - t57410 / F::cast_from(192.0_f64) - t1115 * t54488 / F::cast_from(48.0_f64) - t57415 / F::cast_from(192.0_f64) - t9858 * t4002 / F::cast_from(96.0_f64) - t57422 / F::cast_from(1536.0_f64) - t335 * t338 * t892 * t15273 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57428 + t35057 * t13911 / F::cast_from(48.0_f64) - t57434 / F::cast_from(1536.0_f64) + t8629 * t53939 / F::cast_from(48.0_f64) + t35057 * t13930 / F::cast_from(48.0_f64) - t57441 / F::cast_from(1536.0_f64) + t8793 * t53617 / F::cast_from(24.0_f64);
    t57445
}
