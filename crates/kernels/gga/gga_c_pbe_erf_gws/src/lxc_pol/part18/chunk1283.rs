//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1283/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1283<F: Float>(t1193: F, t353: F, t3717: F, t4386: F, t13925: F, t13930: F, t2409: F, t3066: F, t3189: F, t3207: F, t34922: F, t36129: F, t36323: F, t4164: F, t4183: F, t53187: F, t53189: F, t53199: F, t53224: F, t53231: F, t53261: F, t56265: F, t56267: F, t56269: F, t56276: F, t56282: F, t6793: F, t9283: F) -> F {
    let t56287 = t4386 * t353 * t1193 * t3717;
    let t56294 = -t53187 - t53189 - t53199 + t3066 * t2409 * t36129 * t4164 / F::cast_from(24.0_f64) - t56265 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t56267 - t56269 / F::cast_from(24.0_f64) + t34922 * t13925 / F::cast_from(96.0_f64) + t56276 / F::cast_from(768.0_f64) + t36323 * t13930 / F::cast_from(48.0_f64) + t6793 * t56282 / F::cast_from(24.0_f64) + t6793 * t56287 / F::cast_from(48.0_f64) - t53224 + t53231 - t3207 * t9283 * t4183 * t3189 / F::cast_from(8.0_f64) - t53261;
    t56294
}
