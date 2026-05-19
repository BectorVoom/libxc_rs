//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 916/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk916<F: Float>(t8406: F, t8409: F, t8413: F, t8417: F, t8420: F, t8423: F, t8428: F, t8431: F, t8433: F, t8435: F, t8437: F, t8444: F, t8446: F, t8455: F, t8457: F, t8461: F, t8463: F, t8467: F, t8471: F, t8473: F, t8476: F, t8479: F) -> (F, F) {
    let t10470 = -F::cast_from(0.5503555378190714909e-3_f64) * t8406 + F::cast_from(0.86898242813537603826e-4_f64) * t8409 + F::cast_from(0.41711156550498049836e-2_f64) * t8413 + F::cast_from(0.13903718850166016612e-3_f64) * t8417 - F::cast_from(0.40491142209179332048e-4_f64) * t8420 + F::cast_from(0.19570718734436677156e-3_f64) * t8423 + F::cast_from(0.24720812115595177536e-3_f64) * t8428 + F::cast_from(0.12147342662753799615e-3_f64) * t8431 + F::cast_from(0.92691459001106777413e-2_f64) * t8433 - F::cast_from(0.15448576500184462902e-2_f64) * t8435 + F::cast_from(0.5503555378190714909e-3_f64) * t8437;
    let t10484 = -F::cast_from(0.24720812115595177536e-3_f64) * t8444 + F::cast_from(0.14036929299182168444e-2_f64) * t8446 + F::cast_from(0.86113974316397016943e-6_f64) * t8455 - F::cast_from(0.1545050757224698596e-4_f64) * t8457 - F::cast_from(0.86898242813537603826e-5_f64) * t8461 + F::cast_from(0.10815355300572890172e-3_f64) * t8463 - F::cast_from(0.41711156550498049836e-2_f64) * t8467 + F::cast_from(0.12147342662753799615e-3_f64) * t8471 - F::cast_from(0.41711156550498049836e-2_f64) * t8473 - F::cast_from(0.13903718850166016612e-3_f64) * t8476 - F::cast_from(0.41711156550498049836e-2_f64) * t8479;
    (t10470, t10484)
}
