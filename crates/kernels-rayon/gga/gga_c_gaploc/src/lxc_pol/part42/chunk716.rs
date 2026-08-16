//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 716/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk716(t14267: f64, t531: f64, t14266: f64, t189: f64, t188: f64, t12940: f64, t13417: f64, t13423: f64, t13424: f64, t13425: f64, t13428: f64, t13436: f64, t13440: f64, t13789: f64, t13793: f64, t193: f64, t557: f64) -> (f64, f64, f64, f64) {
    let t14331 = t531 * t14267;
    let t14334 = t189 * t14266;
    let t14335 = t188 * t14334;
    let t14338 = -0.63904876589867916127e-1_f64 * t12940 + t13417 - t13423 + 0.59584149919750711116e-1_f64 * t13789 - 0.59584149919750711116e-1_f64 * t13793 - t13424 - t13425 - 0.35750489951850426669e0_f64 * t557 * t14331 + 0.35750489951850426669e0_f64 * t14335 * t193 - t13428 + t13436 - t13440;
    (t14331, t14334, t14335, t14338)
}
