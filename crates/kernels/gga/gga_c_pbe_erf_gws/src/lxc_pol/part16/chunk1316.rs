//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1316/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1316<F: Float>(t1161: F, t1206: F, t14240: F, t14250: F, t14881: F, t2409: F, t2417: F, t3066: F, t3067: F, t3207: F, t4227: F, t53323: F, t53327: F, t53338: F, t55059: F, t55062: F, t55065: F, t55074: F, t55077: F, t55087: F, t55090: F, t6793: F, t8589: F, t8647: F, t8759: F, t9283: F, t9296: F) -> F {
    let t55093 = -t3207 * t2409 * t8589 * t14250 / F::cast_from(16.0_f64) - t3066 * t2409 * t9296 * t4227 * t2417 / F::cast_from(16.0_f64) + F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t55059 - t55062 - t53323 / F::cast_from(384.0_f64) + t6793 * t55065 / F::cast_from(24.0_f64) - t53327 / F::cast_from(192.0_f64) + t3066 * t2409 * t3067 * t14240 * t1161 / F::cast_from(48.0_f64) - t55074 + t53338 / F::cast_from(768.0_f64) - t55077 - t3207 * t9283 * t1206 * t8759 / F::cast_from(16.0_f64) - t3066 * t9283 * t14881 * t8647 / F::cast_from(8.0_f64) + t55087 - t6793 * t55090 / F::cast_from(12.0_f64);
    t55093
}
