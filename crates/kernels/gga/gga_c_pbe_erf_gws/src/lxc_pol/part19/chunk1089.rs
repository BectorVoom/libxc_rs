//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1089/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1089<F: Float>(t12166: F, t12171: F, t12182: F, t12187: F, t12191: F, t12195: F, t335: F, t6731: F, t6793: F, t844: F, t8602: F, t8629: F, t8690: F, t8700: F, t8716: F, t8793: F, t9249: F, t9253: F, t9272: F, t9275: F, t9289: F, t9290: F) -> F {
    let t12197 = t335 * t12166 / F::new(96.0) - t844 * t12171 / F::new(48.0) + t8629 * t8690 / F::new(48.0) + t8793 * t8602 / F::new(8.0) + t8793 * t8716 / F::new(24.0) + t6793 * t12182 / F::new(24.0) + t8629 * t8700 / F::new(24.0) - t9249 + t9253 - t6731 - F::new(7.0) / F::new(48.0) * t12187 - t9272 + F::new(35.0) / F::new(216.0) * t9275 + t9289 - t844 * t12191 / F::new(48.0) - F::new(35.0) / F::new(108.0) * t9290 - F::new(7.0) / F::new(144.0) * t12195;
    t12197
}
