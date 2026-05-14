//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1052/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1052<F: Float>(t15050: F, t15057: F, t15218: F, t15220: F, t15222: F, t15224: F, t15226: F, t15228: F, t15230: F, t15232: F, t15234: F, t15236: F, t15238: F, t15241: F, t15243: F, t15245: F) -> (F,) {
    let t15247 = t15218 / 768.0 + t15220 / 96.0 - t15222 / 96.0 + t15224 / 768.0 + t15226 / 96.0 - t15228 / 48.0 - t15230 / 768.0 + t15232 / 256.0 - t15234 / 768.0 + t15236 / 24.0 - t15238 / 24.0 - t15241 / 96.0 - t15243 / 768.0 + t15050 - t15057 + t15245 / 96.0;
    (t15247,)
}
