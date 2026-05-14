//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1073/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1073<F: Float>(t169: F, t3081: F, t35194: F, t11412: F, t26447: F, t27624: F, t11431: F, t27754: F, t1030: F, t27597: F, t34026: F, t21825: F, t3680: F, t35302: F, t35304: F, t35307: F, t35309: F, t35312: F, t35316: F) -> (F,) {
    let t35319 = t169 * t35194 * t3081;
    let t35323 = t169 * t11412 * t26447 * t27624;
    let t35325 = t11431 * t27754;
    let t35328 = t1030 * t34026 * t27597;
    let t35330 = t21825 * t3680;
    let t35332 = 0.10821235962619981449e-3 * t35302 - 0.34416463048299153652e-7 * t35304 - 0.8433973524305555556e-6 * t35307 - 0.40483072916666666668e-4 * t35309 + 0.24458523220486111112e-4 * t35312 - 0.19323635647535681159e-7 * t35316 + 0.74218967013888888891e-4 * t35319 + 0.24599089445891203706e-6 * t35323 + 0.30775559784820528656e-8 * t35325 + 0.8976204937239320858e-9 * t35328 + 0.10860115658064651693e-4 * t35330;
    (t35332,)
}
