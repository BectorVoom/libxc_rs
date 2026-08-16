//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1289/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1289<F: Float>(t14733: F, t8690: F, t11407: F, t14797: F, t3989: F, t3990: F, t12237: F, t13780: F, t14637: F, t12213: F, t12220: F, t12248: F, t13888: F, t14667: F, t15360: F, t2376: F, t2408: F, t2409: F, t2494: F, t27729: F, t3066: F, t4001: F, t4182: F, t53354: F, t56333: F, t56337: F, t56341: F, t56343: F, t56349: F, t56351: F, t56357: F, t8734: F, t9283: F) -> F {
    let t56362 = t14733 * t8690;
    let t56366 = t3989 * t3990 * t14797 * t11407;
    let t56374 = t14637 * t3990 * t13780 * t12237;
    let t56381 = t56333 / F::cast_from(768.0_f64) + t56337 / F::cast_from(384.0_f64) + t56341 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t56343 + t3066 * t2409 * t12213 * t14667 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t56349 + t56351 / F::cast_from(96.0_f64) - t2408 * t9283 * t13888 * t12248 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t56357 - t12220 * t27729 * t4001 / F::cast_from(96.0_f64) + t56362 / F::cast_from(48.0_f64) + t53354 + t56366 / F::cast_from(768.0_f64) + t3066 * t2409 * t8734 * t15360 / F::cast_from(24.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t56374 + t2408 * t2409 * t2376 * t4182 * t2494 / F::cast_from(24.0_f64);
    t56381
}
