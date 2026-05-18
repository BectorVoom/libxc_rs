//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 725/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk725<F: Float>(t353: F, t4111: F, t338: F, t2408: F, t3066: F, t335: F, t3953: F, t3961: F, t3963: F, t3967: F, t3977: F, t3986: F, t3994: F, t3998: F, t4072: F, t4077: F, t4083: F, t4087: F, t4090: F, t4094: F, t4099: F, t827: F) -> (F, F) {
    let t4112 = t353 * t4111;
    let t4113 = t338 * t4112;
    let t4116 = t3953 / F::new(48.0) - t4072 - t3961 / F::new(24.0) + t3963 / F::new(48.0) - t3967 / F::new(48.0) + t3977 / F::new(768.0) - t4077 - t3986 / F::new(384.0) - t3994 / F::new(1536.0) - t3998 / F::new(1536.0) - t827 * t4083 / F::new(96.0) + t4087 + t2408 * t4090 / F::new(48.0) - t335 * t4094 / F::new(96.0) + t3066 * t4099 / F::new(48.0) - t335 * t4113 / F::new(96.0);
    (t4113, t4116)
}
