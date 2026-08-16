//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1184/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1184<F: Float>(t15050: F, t15057: F, t15218: F, t15220: F, t15222: F, t15224: F, t15226: F, t15228: F, t15230: F, t15232: F, t15234: F, t15236: F, t15238: F, t15241: F, t15243: F, t15245: F) -> F {
    let t15247 = t15218 / F::cast_from(768.0_f64) + t15220 / F::cast_from(96.0_f64) - t15222 / F::cast_from(96.0_f64) + t15224 / F::cast_from(768.0_f64) + t15226 / F::cast_from(96.0_f64) - t15228 / F::cast_from(48.0_f64) - t15230 / F::cast_from(768.0_f64) + t15232 / F::cast_from(256.0_f64) - t15234 / F::cast_from(768.0_f64) + t15236 / F::cast_from(24.0_f64) - t15238 / F::cast_from(24.0_f64) - t15241 / F::cast_from(96.0_f64) - t15243 / F::cast_from(768.0_f64) + t15050 - t15057 + t15245 / F::cast_from(96.0_f64);
    t15247
}
