//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 785/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk785<F: Float>(t8406: F, t8409: F, t8413: F, t8417: F, t8420: F, t8423: F, t8428: F, t8431: F, t8433: F, t8435: F, t8437: F, t8444: F, t8446: F, t8455: F, t8457: F, t8461: F, t8463: F, t8467: F, t8471: F, t8473: F, t8476: F, t8479: F) -> (F, F) {
    let t10470 = -0.5503555378190714909e-3 * t8406 + 0.86898242813537603826e-4 * t8409 + 0.41711156550498049836e-2 * t8413 + 0.13903718850166016612e-3 * t8417 - 0.40491142209179332048e-4 * t8420 + 0.19570718734436677156e-3 * t8423 + 0.24720812115595177536e-3 * t8428 + 0.12147342662753799615e-3 * t8431 + 0.92691459001106777413e-2 * t8433 - 0.15448576500184462902e-2 * t8435 + 0.5503555378190714909e-3 * t8437;
    let t10484 = -0.24720812115595177536e-3 * t8444 + 0.14036929299182168444e-2 * t8446 + 0.86113974316397016943e-6 * t8455 - 0.1545050757224698596e-4 * t8457 - 0.86898242813537603826e-5 * t8461 + 0.10815355300572890172e-3 * t8463 - 0.41711156550498049836e-2 * t8467 + 0.12147342662753799615e-3 * t8471 - 0.41711156550498049836e-2 * t8473 - 0.13903718850166016612e-3 * t8476 - 0.41711156550498049836e-2 * t8479;
    (t10470, t10484)
}
