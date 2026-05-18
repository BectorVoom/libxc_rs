//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1121/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1121<F: Float>(t13904: F, t13907: F, t13921: F, t13945: F, t13950: F, t13954: F, t13958: F, t13964: F, t13966: F, t13969: F, t14302: F, t14305: F, t14311: F, t14322: F, t14327: F, t2388: F, t2392: F, t2408: F, t4083: F, t827: F) -> F {
    let t14332 = t13904 / F::new(768.0) - t14302 + t13907 / F::new(768.0) - t13921 / F::new(384.0) - F::new(7.0) / F::new(72.0) * t14305 - t2392 * t4083 / F::new(96.0) - t827 * t14311 / F::new(48.0) - t2388 * t4083 / F::new(96.0) + t13945 / F::new(48.0) - t13950 / F::new(12.0) + F::new(7.0) / F::new(72.0) * t13954 - t13958 / F::new(384.0) + F::new(7.0) / F::new(1152.0) * t13964 - t2408 * t14322 / F::new(12.0) - t827 * t14327 / F::new(48.0) + t13966 / F::new(12.0) - t13969 / F::new(24.0);
    t14332
}
