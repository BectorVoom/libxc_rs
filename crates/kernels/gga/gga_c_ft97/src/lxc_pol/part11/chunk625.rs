//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 625/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk625<F: Float>(t8277: F, t9224: F, t1985: F, t2: F, t2075: F, t558: F, t582: F, t8266: F, t7368: F, t24: F, t9017: F, t2118: F, t458: F, t462: F, t92: F, t9205: F, t9207: F, t9209: F, t9211: F, t9214: F, t9218: F, t9221: F) -> (F, F, F, F, F) {
    let t9225 = t9224 * t8277;
    let t9230 = t1985 * t2 * t558 * t2075;
    let t9233 = t582 * t8266;
    let t9236 = t7368 * t2;
    let t9238 = t24 * t9236 * t9017;
    let t9241 = t458 * t2118;
    let t9242 = t9205 / 3.0 + 2.0 / 9.0 * t9207 - 2.0 * t9209 - 2.0 * t462 * t9211 + 2.0 * t462 * t9214 - 2.0 * t462 * t9218 - 2.0 * t462 * t9221 - 10.0 / 27.0 * t462 * t9225 + 6.0 * t462 * t9230 - t462 * t9233 / 3.0 - 6.0 * t92 * t9238 + t9241;
    (t9225, t9230, t9233, t9238, t9242)
}
