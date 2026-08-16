//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1009/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1009(t85542: f64, t85567: f64, t11361: f64, t15805: f64, t15811: f64, t1594: f64, t1599: f64, t1711: f64, t2021: f64, t35: f64, t372: f64, t374: f64, t37996: f64, t38200: f64, t38211: f64, t38242: f64, t409: f64, t4445: f64, t4491: f64, t58513: f64, t64: f64, t6426: f64, t85413: f64, t85414: f64, t85424: f64, t85435: f64, t85439: f64, t85460: f64, t85506: f64) -> (f64, f64) {
    let t85568 = t85542 + t85567;
    let t85573 = 0.6139293849859577088e-2_f64 * t372 * t37996 * t85414 + 0.40531318161212073987e-5_f64 * t2021 * t85413 * t1599 + 0.73006706433865497404e-4_f64 * t38211 * t85413 * t1599 + 0.20279640676073749279e-3_f64 * t1594 * t85424 * t1599 - 0.23238868087529279928e-2_f64 * t11361 * t58513 * t4445 - 0.279058811357253504e0_f64 * t15811 * t6426 * t15805 * t4491 + 24.0_f64 * t64 * t38242 * t85435 + 6.0_f64 * t64 * t1711 * t85439 - t64 * t409 * (t85460 + t85506) + 0.13126093506691345164e-6_f64 * t38200 * t85413 * t1599 - 0.11627450473218896e-1_f64 * t372 * t374 * t85568 * t35;
    (t85568, t85573)
}
