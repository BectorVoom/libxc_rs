//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1237/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1237<F: Float>(t1115: F, t1185: F, t13772: F, t13849: F, t13910: F, t13929: F, t13939: F, t14576: F, t2074: F, t2182: F, t2376: F, t2408: F, t2409: F, t2498: F, t27105: F, t3066: F, t3067: F, t3207: F, t34963: F, t4182: F, t50967: F, t53083: F, t53093: F, t53099: F, t53126: F, t53131: F, t6793: F, t810: F, t8654: F, t938: F) -> F {
    let t53133 = -t6793 * t53083 / F::cast_from(12.0_f64) + t8654 * t27105 * t13929 / F::cast_from(24.0_f64) + t8654 * t1185 * t13910 / F::cast_from(24.0_f64) - t53093 - t3066 * t2409 * t34963 * t13849 / F::cast_from(16.0_f64) - t53099 + t2408 * t2409 * t2376 * t14576 * t810 / F::cast_from(24.0_f64) + t2408 * t2409 * t2376 * t4182 * t2074 / F::cast_from(48.0_f64) - t3207 * t2409 * t2376 * t4182 * t2182 / F::cast_from(16.0_f64) + t3066 * t2409 * t3067 * t14576 * t938 / F::cast_from(24.0_f64) - t2498 * t13939 / F::cast_from(48.0_f64) - t1115 * t50967 / F::cast_from(96.0_f64) - t2498 * t13772 / F::cast_from(48.0_f64) - t53126 / F::cast_from(24.0_f64) - t53131 / F::cast_from(1536.0_f64);
    t53133
}
