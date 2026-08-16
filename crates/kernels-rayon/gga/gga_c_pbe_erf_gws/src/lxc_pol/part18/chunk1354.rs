//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1354/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1354(t11398: f64, t3959: f64, t11757: f64, t3972: f64, t3975: f64, t11588: f64, t14617: f64, t53688: f64, t15195: f64, t9270: f64, t1109: f64, t13925: f64, t14397: f64, t14420: f64, t14437: f64, t22379: f64, t2409: f64, t3066: f64, t3067: f64, t3306: f64, t35000: f64, t35260: f64, t353: f64, t4002: f64, t4053: f64, t4182: f64, t53852: f64, t53874: f64, t53897: f64, t55408: f64, t859: f64, t8629: f64, t8654: f64) -> f64 {
    let t57265 = t3959 * t11398;
    let t57284 = t3972 * t3975 * t11757;
    let t57287 = t3972 * t3975 * t11588;
    let t57289 = t53688 * t14617;
    let t57291 = t9270 * t15195;
    let t57298 = t57265 / 48.0_f64 - 35.0_f64 / 216.0_f64 * t53852 + t35000 * t13925 / 48.0_f64 + t8629 * t859 * t353 * t4053 * t1109 / 96.0_f64 + t22379 * t14420 / 24.0_f64 - t35260 * t4002 / 96.0_f64 - t8654 * t14397 / 48.0_f64 - t8654 * t14437 / 48.0_f64 + t53874 + t55408 + t57284 / 1536.0_f64 + t57287 / 1536.0_f64 - t57289 / 48.0_f64 - t53897 - 7.0_f64 / 72.0_f64 * t57291 + t3066 * t2409 * t3067 * t4182 * t3306 / 24.0_f64;
    t57298
}
