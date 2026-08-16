//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1289/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1289(t14733: f64, t8690: f64, t11407: f64, t14797: f64, t3989: f64, t3990: f64, t12237: f64, t13780: f64, t14637: f64, t12213: f64, t12220: f64, t12248: f64, t13888: f64, t14667: f64, t15360: f64, t2376: f64, t2408: f64, t2409: f64, t2494: f64, t27729: f64, t3066: f64, t4001: f64, t4182: f64, t53354: f64, t56333: f64, t56337: f64, t56341: f64, t56343: f64, t56349: f64, t56351: f64, t56357: f64, t8734: f64, t9283: f64) -> f64 {
    let t56362 = t14733 * t8690;
    let t56366 = t3989 * t3990 * t14797 * t11407;
    let t56374 = t14637 * t3990 * t13780 * t12237;
    let t56381 = t56333 / 768.0_f64 + t56337 / 384.0_f64 + t56341 / 384.0_f64 + 7.0_f64 / 4608.0_f64 * t56343 + t3066 * t2409 * t12213 * t14667 / 24.0_f64 + 7.0_f64 / 4608.0_f64 * t56349 + t56351 / 96.0_f64 - t2408 * t9283 * t13888 * t12248 / 24.0_f64 - 7.0_f64 / 576.0_f64 * t56357 - t12220 * t27729 * t4001 / 96.0_f64 + t56362 / 48.0_f64 + t53354 + t56366 / 768.0_f64 + t3066 * t2409 * t8734 * t15360 / 24.0_f64 - 5.0_f64 / 768.0_f64 * t56374 + t2408 * t2409 * t2376 * t4182 * t2494 / 24.0_f64;
    t56381
}
