//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1368/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1368(t1206: f64, t12164: f64, t15490: f64, t2409: f64, t3060: f64, t3207: f64, t335: f64, t338: f64, t36200: f64, t36201: f64, t4083: f64, t4207: f64, t53666: f64, t55315: f64, t56708: f64, t56717: f64, t56722: f64, t56724: f64, t56728: f64, t56740: f64, t56743: f64, t56745: f64, t56747: f64, t6781: f64, t9858: f64) -> f64 {
    let t58327 = -t56708 / 192.0_f64 + t55315 - t56717 / 192.0_f64 - t56722 / 768.0_f64 - t9858 * t4083 / 96.0_f64 + t56724 / 12.0_f64 + 7.0_f64 / 576.0_f64 * t56728 - t53666 - t335 * t338 * t12164 * t1206 / 96.0_f64 - t56740 / 48.0_f64 + t36200 * t36201 * t4207 * t3060 / 4.0_f64 - t56743 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t56745 - 7.0_f64 / 1152.0_f64 * t56747 - t3207 * t2409 * t6781 * t15490 / 16.0_f64;
    t58327
}
