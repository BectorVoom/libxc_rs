//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1133/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1133<F: Float>(t12237: F, t13780: F, t14637: F, t3990: F, t12213: F, t12220: F, t12248: F, t13888: F, t14667: F, t15360: F, t2376: F, t2408: F, t2409: F, t2494: F, t27729: F, t3066: F, t4001: F, t4182: F, t53354: F, t56333: F, t56337: F, t56341: F, t56343: F, t56349: F, t56351: F, t56357: F, t56362: F, t56366: F, t8734: F, t9283: F) -> (F,) {
    let t56374 = t14637 * t3990 * t13780 * t12237;
    let t56381 = t56333 / 768.0 + t56337 / 384.0 + t56341 / 384.0 + 7.0 / 4608.0 * t56343 + t3066 * t2409 * t12213 * t14667 / 24.0 + 7.0 / 4608.0 * t56349 + t56351 / 96.0 - t2408 * t9283 * t13888 * t12248 / 24.0 - 7.0 / 576.0 * t56357 - t12220 * t27729 * t4001 / 96.0 + t56362 / 48.0 + t53354 + t56366 / 768.0 + t3066 * t2409 * t8734 * t15360 / 24.0 - 5.0 / 768.0 * t56374 + t2408 * t2409 * t2376 * t4182 * t2494 / 24.0;
    (t56381,)
}
