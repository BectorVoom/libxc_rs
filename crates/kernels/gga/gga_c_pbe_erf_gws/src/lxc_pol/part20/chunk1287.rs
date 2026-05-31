//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1287/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1287<F: Float>(t13984: F, t56320: F, t13972: F, t15371: F, t1105: F, t14576: F, t2376: F, t2408: F, t2409: F, t53273: F, t53302: F, t53308: F, t55074: F, t56299: F, t56302: F, t56305: F, t56307: F, t56309: F, t56312: F, t56316: F, t56318: F) -> F {
    let t56321 = t56320 * t13984;
    let t56323 = t13972 * t15371;
    let t56330 = t56299 / F::cast_from(512.0_f64) + t56302 / F::cast_from(1536.0_f64) + t56305 / F::cast_from(384.0_f64) - t56307 / F::cast_from(48.0_f64) - t56309 / F::cast_from(24.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t56312 - t56316 / F::cast_from(96.0_f64) - t56318 / F::cast_from(24.0_f64) - t56321 / F::cast_from(96.0_f64) + t53273 - t53302 - t53308 - t55074 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t56323 + t2408 * t2409 * t2376 * t14576 * t1105 / F::cast_from(24.0_f64);
    t56330
}
