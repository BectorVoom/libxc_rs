//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1239/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1239<F: Float>(t1030: F, t27597: F, t34026: F, t21825: F, t3680: F, t35302: F, t35304: F, t35307: F, t35309: F, t35312: F, t35316: F, t35319: F, t35323: F, t35325: F) -> F {
    let t35328 = t1030 * t34026 * t27597;
    let t35330 = t21825 * t3680;
    let t35332 = F::cast_from(0.10821235962619981449e-3_f64) * t35302 - F::cast_from(0.34416463048299153652e-7_f64) * t35304 - F::cast_from(0.8433973524305555556e-6_f64) * t35307 - F::cast_from(0.40483072916666666668e-4_f64) * t35309 + F::cast_from(0.24458523220486111112e-4_f64) * t35312 - F::cast_from(0.19323635647535681159e-7_f64) * t35316 + F::cast_from(0.74218967013888888891e-4_f64) * t35319 + F::cast_from(0.24599089445891203706e-6_f64) * t35323 + F::cast_from(0.30775559784820528656e-8_f64) * t35325 + F::cast_from(0.8976204937239320858e-9_f64) * t35328 + F::cast_from(0.10860115658064651693e-4_f64) * t35330;
    t35332
}
