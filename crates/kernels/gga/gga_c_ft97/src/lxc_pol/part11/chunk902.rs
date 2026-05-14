//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 902/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk902<F: Float>(t1882: F, t9402: F, t157: F, t40424: F, t8392: F, t9100: F, t2144: F, t8232: F, t376: F, t89: F, t9396: F, t605: F, t9114: F, t12968: F, t144: F, t160: F, t1643: F, t1647: F, t167: F, t1901: F, t2190: F, t2205: F, t379: F, t38071: F, t40594: F, t446: F, t558: F, t569: F, t604: F, t616: F, t7966: F, t9017: F, t9144: F, t9316: F) -> (F,) {
    let t41246 = t1882 * t9402;
    let t41251 = t40424 * t157;
    let t41262 = t8392 * t9100;
    let t41264 = t8232 * t2144;
    let t41267 = t89 * t376 * t9396;
    let t41269 = t9114 * t605;
    let t41278 = -8.0 / 3.0 * t446 * t569 * t616 * t7966 - 8.0 / 3.0 * t446 * t2205 * t167 * t38071 - 8.0 / 3.0 * t41246 + 8.0 * t446 * t144 * t40594 + 8.0 / 3.0 * t1901 * t41251 * t160 * t9017 * t379 - 8.0 * t1901 * t12968 * t604 * t558 * t9316 - 8.0 / 9.0 * t41262 - 16.0 / 9.0 * t41264 - 4.0 / 9.0 * t41267 - 8.0 / 9.0 * t1901 * t41269 * t1643 * t2190 + 8.0 / 3.0 * t1901 * t9144 * t1647 * t2190;
    (t41278,)
}
