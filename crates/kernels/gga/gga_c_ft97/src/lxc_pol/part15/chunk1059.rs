//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1059/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1059<F: Float>(t5457: F, t5468: F, t10915: F, t18926: F, t2938: F, t43050: F, t43084: F, t43250: F, t631: F, t69265: F, t69289: F, t69291: F, t82077: F, t82079: F, t82088: F, t82095: F, t82097: F, t88252: F, t898: F) -> (F,) {
    let t91251 = t5457 * t5457;
    let t91264 = t5468 * t5468;
    let t91269 = 12.0 * t82077 - 4.0 / 9.0 * t82079 - 16.0 / 81.0 * t82088 + 10.0 / 9.0 * t69265 - 8.0 / 3.0 * t82095 + 8.0 / 9.0 * t82097 - 20.0 / 9.0 * t69289 - 10.0 * t69291 - 30.0 * t631 * t898 * t43250 * t91251 - t43084 + 36.0 * t631 * t898 * t18926 * t5468 - 8.0 / 9.0 * t631 * t10915 * t43050 * t88252 - 9.0 / 2.0 * t631 * t898 * t2938 * t91264;
    (t91269,)
}
