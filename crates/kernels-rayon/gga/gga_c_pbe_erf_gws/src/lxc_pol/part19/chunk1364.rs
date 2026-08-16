//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1364/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1364(t1144: f64, t15034: f64, t859: f64, t12237: f64, t14185: f64, t14952: f64, t15526: f64, t2408: f64, t2409: f64, t3066: f64, t3189: f64, t3207: f64, t4228: f64, t53544: f64, t55212: f64, t55218: f64, t55228: f64, t56548: f64, t56551: f64, t56553: f64, t56555: f64, t56560: f64, t56578: f64, t6793: f64, t8589: f64, t8734: f64, t9283: f64) -> f64 {
    let t58201 = t859 * t1144 * t15034;
    let t58224 = -t56548 / 384.0_f64 + t56551 / 96.0_f64 + t6793 * t58201 / 24.0_f64 + t55212 - 35.0_f64 / 576.0_f64 * t56553 + t55218 + t56555 / 24.0_f64 - t3207 * t9283 * t4228 * t3189 / 8.0_f64 + t55228 + t3207 * t9283 * t14185 * t12237 / 8.0_f64 - 7.0_f64 / 1152.0_f64 * t56560 - t53544 + t2408 * t2409 * t8589 * t14952 / 24.0_f64 + t3066 * t2409 * t8734 * t15526 / 48.0_f64 + t56578 / 48.0_f64;
    t58224
}
