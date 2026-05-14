//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 962/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk962<F: Float>(t30105: F, t525: F, t165: F, t28: F, t26791: F, t6587: F, t1017: F, t1058: F, t5778: F, t4714: F, t1384: F, t4724: F, t9439: F, t1053: F, t6718: F, t2179: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t30106 = t525 * t30105;
    let t30107 = t30106 * t165;
    let t30108 = t28 * t30107;
    let t30111 = t26791 * t6587;
    let t30112 = t28 * t30111;
    let t30117 = t1058 * t1017;
    let t30118 = t5778 * t30117;
    let t30119 = t28 * t30118;
    let t30122 = t165 * t4714;
    let t30123 = t5778 * t30122;
    let t30124 = t28 * t30123;
    let t30127 = t1384 * t4724;
    let t30128 = t9439 * t30127;
    let t30130 = t6718 * t1053;
    let t30131 = t2179 * t30130;
    (t30106, t30107, t30108, t30111, t30112, t30117, t30118, t30119, t30122, t30123, t30124, t30127, t30128, t30130, t30131)
}
