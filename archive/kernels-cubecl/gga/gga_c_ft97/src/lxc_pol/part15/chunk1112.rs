//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1112/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1112<F: Float>(t42087: F, t88252: F, t9920: F, t2497: F, t88239: F, t42095: F, t9953: F, t1131: F, t21655: F, t13688: F, t13689: F, t13693: F, t18293: F, t21457: F, t2372: F, t2486: F, t42094: F, t462: F, t5053: F, t67078: F, t67097: F, t737: F, t80963: F, t88240: F, t88248: F, t9707: F, t9952: F) -> (F, F, F, F, F, F, F) {
    let t88253 = t42087 * t88252;
    let t88257 = t9920 * t88252;
    let t88261 = t2497 * t88239;
    let t88269 = t42095 * t88252;
    let t88273 = t9953 * t88252;
    let t88277 = t21655 * t1131;
    let t88286 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t2486 * t88240 - F::cast_from(36.0_f64) * t462 * t9707 * t18293 * t5053 - t462 * t737 * t88248 / F::cast_from(3.0_f64) + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t462 * t9952 * t88253 + F::cast_from(8.0_f64) * t462 * t737 * t88257 + F::cast_from(2.0_f64) * t462 * t737 * t88261 + F::cast_from(8.0_f64) * t462 * t2372 * t80963 * t1131 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t462 * t42094 * t88269 - F::cast_from(8.0_f64) * t462 * t2486 * t88273 - F::cast_from(8.0_f64) * t13688 * t13689 * t88277 - F::cast_from(8.0_f64) * t13688 * t13693 * t21457 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t67078 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t67097;
    (t88253, t88257, t88261, t88269, t88273, t88277, t88286)
}
