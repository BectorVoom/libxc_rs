//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 716/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk716<F: Float>(t14267: F, t531: F, t14266: F, t189: F, t188: F, t12940: F, t13417: F, t13423: F, t13424: F, t13425: F, t13428: F, t13436: F, t13440: F, t13789: F, t13793: F, t193: F, t557: F) -> (F, F, F, F) {
    let t14331 = t531 * t14267;
    let t14334 = t189 * t14266;
    let t14335 = t188 * t14334;
    let t14338 = -F::cast_from(0.63904876589867916127e-1_f64) * t12940 + t13417 - t13423 + F::cast_from(0.59584149919750711116e-1_f64) * t13789 - F::cast_from(0.59584149919750711116e-1_f64) * t13793 - t13424 - t13425 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t14331 + F::cast_from(0.35750489951850426669e0_f64) * t14335 * t193 - t13428 + t13436 - t13440;
    (t14331, t14334, t14335, t14338)
}
