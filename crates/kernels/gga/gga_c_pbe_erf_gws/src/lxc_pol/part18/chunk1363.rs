//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1363/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1363<F: Float>(t15292: F, t840: F, t361: F, t57321: F, t13917: F, t3223: F, t12014: F, t13919: F, t1115: F, t13911: F, t13930: F, t14397: F, t15273: F, t2498: F, t335: F, t338: F, t35057: F, t4002: F, t53617: F, t53939: F, t54488: F, t57402: F, t57404: F, t57410: F, t57415: F, t57422: F, t8629: F, t8793: F, t892: F, t9858: F) -> F {
    let t57428 = t840 * t15292;
    let t57432 = t361 * t57321;
    let t57434 = t13917 * t57432 * t3223;
    let t57441 = t13917 * t13919 * t12014;
    let t57445 = t57402 / F::new(24.0) + t57404 / F::new(24.0) - t2498 * t14397 / F::new(48.0) - t57410 / F::new(192.0) - t1115 * t54488 / F::new(48.0) - t57415 / F::new(192.0) - t9858 * t4002 / F::new(96.0) - t57422 / F::new(1536.0) - t335 * t338 * t892 * t15273 / F::new(96.0) + F::new(7.0) / F::new(144.0) * t57428 + t35057 * t13911 / F::new(48.0) - t57434 / F::new(1536.0) + t8629 * t53939 / F::new(48.0) + t35057 * t13930 / F::new(48.0) - t57441 / F::new(1536.0) + t8793 * t53617 / F::new(24.0);
    t57445
}
