//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1032/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1032<F: Float>(t1115: F, t2384: F, t2397: F, t2408: F, t2503: F, t3040: F, t3066: F, t3079: F, t3207: F, t4419: F, t6746: F, t6748: F, t6805: F, t844: F, t9275: F, t9285: F, t9289: F, t9290: F, t9293: F, t9299: F, t9302: F, t9307: F, t9313: F, t9317: F) -> F {
    let t9320 = F::new(35.0) / F::new(432.0) * t9275 + t3040 * t2397 / F::new(48.0) - F::new(7.0) / F::new(288.0) * t6746 + t2384 * t2503 / F::new(96.0) - F::new(7.0) / F::new(144.0) * t6748 - t2408 * t9285 / F::new(12.0) + t9289 - F::new(35.0) / F::new(216.0) * t9290 - t3207 * t9293 / F::new(8.0) - t3066 * t9299 / F::new(16.0) + t9302 * t3079 / F::new(96.0) + F::new(7.0) / F::new(72.0) * t6805 - t844 * t9307 / F::new(48.0) + t1115 * t4419 / F::new(96.0) - t844 * t9313 / F::new(24.0) - t844 * t9317 / F::new(24.0);
    t9320
}
