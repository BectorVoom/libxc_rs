//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 888/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk888<F: Float>(t85542: F, t85567: F, t11361: F, t15805: F, t15811: F, t1594: F, t1599: F, t1711: F, t2021: F, t35: F, t372: F, t374: F, t37996: F, t38200: F, t38211: F, t38242: F, t409: F, t4445: F, t4491: F, t58513: F, t64: F, t6426: F, t85413: F, t85414: F, t85424: F, t85435: F, t85439: F, t85460: F, t85506: F) -> (F, F) {
    let t85568 = t85542 + t85567;
    let t85573 = 0.6139293849859577088e-2 * t372 * t37996 * t85414 + 0.40531318161212073987e-5 * t2021 * t85413 * t1599 + 0.73006706433865497404e-4 * t38211 * t85413 * t1599 + 0.20279640676073749279e-3 * t1594 * t85424 * t1599 - 0.23238868087529279928e-2 * t11361 * t58513 * t4445 - 0.279058811357253504e0 * t15811 * t6426 * t15805 * t4491 + 24.0 * t64 * t38242 * t85435 + 6.0 * t64 * t1711 * t85439 - t64 * t409 * (t85460 + t85506) + 0.13126093506691345164e-6 * t38200 * t85413 * t1599 - 0.11627450473218896e-1 * t372 * t374 * t85568 * t35;
    (t85568, t85573)
}
