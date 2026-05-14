//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 906/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk906<F: Float>(t1903: F, t1907: F, t1914: F, t5441: F, t256: F, t5444: F, t719: F, t16580: F, t17467: F, t17469: F, t17473: F, t17476: F, t17481: F, t17484: F, t17488: F, t247: F, t251: F) -> (F,) {
    let t18284 = t1907 * t1903;
    let t18286 = t1914 * t5441;
    let t18293 = t5444 * t719 * t256;
    let t18295 = t17467 - t17469 + t17473 - t17476 - t17481 + t17484 - t17488 - 4.0 / 9.0 * t18284 - 0.5402469135802469136e-1 * t18286 + t16580 * t247 * t251 * t256 / 3.0 + 4.0 / 3.0 * t18293;
    (t18295,)
}
