//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 995/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk995<F: Float>(t13313: F, t2493: F, t42164: F, t42206: F, t462: F, t67329: F, t67331: F, t81010: F, t81040: F, t81042: F, t88141: F, t88145: F, t88157: F, t88176: F, t88188: F, t88219: F, t88223: F, t88612: F, t9896: F, t9916: F) -> (F,) {
    let t88983 = -8.0 / 3.0 * t81010 + 16.0 / 3.0 * t67329 - 8.0 / 3.0 * t67331 - 4.0 * t462 * t9896 * t88223 + 40.0 / 27.0 * t462 * t42164 * t88141 + 4.0 / 3.0 * t462 * t2493 * t88219 + 4.0 / 3.0 * t462 * t2493 * t88145 + t42206 + 8.0 * t81040 + 4.0 / 3.0 * t81042 + 4.0 / 3.0 * t462 * t9916 * t88188 - 4.0 * t462 * t2493 * t88157 - 20.0 / 9.0 * t462 * t13313 * t88612 + 8.0 * t462 * t2493 * t88176;
    (t88983,)
}
