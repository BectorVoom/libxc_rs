//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1281/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1281<F: Float>(t2376: F, t2408: F, t2409: F, t3717: F, t4052: F, t53093: F, t53099: F, t53155: F, t53177: F, t53179: F, t53220: F, t56228: F, t56236: F, t56240: F, t56242: F, t56250: F, t56252: F, t56255: F, t56257: F, t8793: F) -> F {
    let t56259 = t8793 * t53220 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t56228 + t2408 * t2409 * t2376 * t4052 * t3717 / F::cast_from(48.0_f64) - t56236 / F::cast_from(12.0_f64) + t56240 / F::cast_from(1536.0_f64) - t53093 - t53099 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t56242 - t53155 + t56250 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t56252 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t56255 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t56257 - t53177 - t53179;
    t56259
}
