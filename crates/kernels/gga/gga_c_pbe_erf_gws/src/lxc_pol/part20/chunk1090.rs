//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1090/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1090<F: Float>(t12166: F, t12171: F, t12182: F, t12187: F, t12191: F, t12195: F, t335: F, t6731: F, t6793: F, t844: F, t8602: F, t8629: F, t8690: F, t8700: F, t8716: F, t8793: F, t9249: F, t9253: F, t9272: F, t9275: F, t9289: F, t9290: F) -> F {
    let t12197 = t335 * t12166 / F::cast_from(96.0_f64) - t844 * t12171 / F::cast_from(48.0_f64) + t8629 * t8690 / F::cast_from(48.0_f64) + t8793 * t8602 / F::cast_from(8.0_f64) + t8793 * t8716 / F::cast_from(24.0_f64) + t6793 * t12182 / F::cast_from(24.0_f64) + t8629 * t8700 / F::cast_from(24.0_f64) - t9249 + t9253 - t6731 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t12187 - t9272 + F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t9275 + t9289 - t844 * t12191 / F::cast_from(48.0_f64) - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t9290 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t12195;
    t12197
}
