//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1249/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1249<F: Float>(t4146: F, t51818: F, t14797: F, t3989: F, t3990: F, t9321: F, t13781: F, t14582: F, t3972: F, t9380: F, t12213: F, t13840: F, t14446: F, t2352: F, t2376: F, t2408: F, t2409: F, t2494: F, t26654: F, t27112: F, t3066: F, t3067: F, t3306: F, t4007: F, t4052: F, t4164: F, t4182: F, t53299: F, t53302: F, t53308: F, t53323: F, t53327: F, t6781: F) -> F {
    let t53334 = t51818 * t4146;
    let t53338 = t3989 * t3990 * t14797 * t9321;
    let t53346 = t3972 * t13781 * t14582 * t9380;
    let t53348 = t2408 * t2409 * t2376 * t4052 * t2494 / F::cast_from(24.0_f64) + t53299 / F::cast_from(768.0_f64) - t53302 + t3066 * t2409 * t27112 * t4164 / F::cast_from(48.0_f64) - t53308 + t2408 * t2409 * t6781 * t14446 / F::cast_from(24.0_f64) + t2408 * t2409 * t26654 * t4007 / F::cast_from(24.0_f64) + t3066 * t2409 * t3067 * t4182 * t2352 / F::cast_from(48.0_f64) - t53323 / F::cast_from(768.0_f64) - t53327 / F::cast_from(384.0_f64) + t3066 * t2409 * t3067 * t4052 * t3306 / F::cast_from(24.0_f64) - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t53334 + t53338 / F::cast_from(1536.0_f64) + t3066 * t2409 * t12213 * t13840 / F::cast_from(48.0_f64) - t53346 / F::cast_from(1536.0_f64);
    t53348
}
