//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1314/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1314<F: Float>(t53260: F, t53272: F, t1205: F, t12213: F, t14289: F, t2376: F, t2408: F, t2409: F, t3066: F, t52217: F, t53243: F, t53246: F, t53248: F, t53251: F, t53264: F, t53266: F, t53270: F, t53276: F, t53286: F, t53299: F, t8574: F) -> F {
    let t55031 = F::new(7.0) / F::new(72.0) * t53260;
    let t55036 = F::new(7.0) / F::new(72.0) * t53272;
    let t55049 = -t53243 / F::new(384.0) - t53246 / F::new(12.0) - t53248 / F::new(48.0) - t53251 / F::new(24.0) - t55031 + F::new(7.0) / F::new(36.0) * t52217 + t53264 / F::new(1536.0) + t53266 / F::new(24.0) + t53270 / F::new(256.0) + t55036 - t53276 / F::new(1536.0) - t53286 / F::new(256.0) + t3066 * t2409 * t12213 * t14289 / F::new(48.0) + t2408 * t2409 * t2376 * t1205 * t8574 / F::new(48.0) + t53299 / F::new(384.0);
    t55049
}
