//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1213/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1213<F: Float>(t5457: F, t5468: F, t10915: F, t18926: F, t2938: F, t43050: F, t43084: F, t43250: F, t631: F, t69265: F, t69289: F, t69291: F, t82077: F, t82079: F, t82088: F, t82095: F, t82097: F, t88252: F, t898: F) -> F {
    let t91251 = t5457 * t5457;
    let t91264 = t5468 * t5468;
    let t91269 = F::cast_from(12.0_f64) * t82077 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t82079 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t82088 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t69265 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t82095 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t82097 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t69289 - F::cast_from(10.0_f64) * t69291 - F::cast_from(30.0_f64) * t631 * t898 * t43250 * t91251 - t43084 + F::cast_from(36.0_f64) * t631 * t898 * t18926 * t5468 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t631 * t10915 * t43050 * t88252 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t631 * t898 * t2938 * t91264;
    t91269
}
