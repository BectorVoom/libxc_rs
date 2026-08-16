//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1373/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1373(t15425: f64, t4414: f64, t1205: f64, t12098: f64, t12213: f64, t14935: f64, t15443: f64, t2409: f64, t3066: f64, t3067: f64, t53886: f64, t55382: f64, t55385: f64, t55403: f64, t55420: f64, t55421: f64, t57260: f64, t57262: f64, t57265: f64, t57284: f64, t57287: f64, t57289: f64, t8734: f64) -> f64 {
    let t58479 = t4414 * t15425;
    let t58488 = t3066 * t2409 * t12213 * t14935 / 24.0_f64 + t3066 * t2409 * t8734 * t15443 / 24.0_f64 + t3066 * t2409 * t3067 * t1205 * t12098 / 48.0_f64 - 7.0_f64 / 72.0_f64 * t58479 - t55382 + 7.0_f64 / 144.0_f64 * t57260 + t57262 / 12.0_f64 + t55385 + t57265 / 24.0_f64 + t55403 + 119.0_f64 / 1728.0_f64 * t53886 + t57284 / 768.0_f64 + t57287 / 768.0_f64 - t57289 / 24.0_f64 + t55420 - t55421;
    t58488
}
