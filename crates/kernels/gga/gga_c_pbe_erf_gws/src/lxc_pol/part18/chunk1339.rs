//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1339/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1339<F: Float>(t54258: F, t54260: F, t57082: F, t57086: F, t57088: F, t57090: F, t57092: F, t57094: F, t57096: F, t57098: F, t57100: F, t57102: F, t57104: F) -> F {
    let t57106 = -t57082 / F::new(768.0) + t57086 / F::new(48.0) - t57088 / F::new(24.0) - t57090 / F::new(96.0) - t57092 / F::new(768.0) - F::new(5.0) / F::new(192.0) * t57094 + t57096 / F::new(96.0) + t57098 / F::new(48.0) + t54258 + t57100 / F::new(96.0) - t57102 / F::new(96.0) - t54260 + F::new(7.0) / F::new(1152.0) * t57104;
    t57106
}
