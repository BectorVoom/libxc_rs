//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 979/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk979<F: Float>(t1329: F, t2: F, t555: F, t2091: F, t4665: F, t636: F, t13515: F, t108: F, t105: F, t13181: F, t13202: F, t1327: F, t13501: F, t13505: F, t13511: F, t13516: F, t13526: F, t3525: F, t3529: F, t4650: F, t4653: F, t4656: F, t631: F, t637: F, t97: F) -> F {
    let t13529 = t1329 * t2;
    let t13530 = t13529 * t555;
    let t13533 = t2091 * t4665;
    let t13534 = t13533 * t636;
    let t13537 = -t13515;
    let t13538 = t108 * t13537;
    let t13541 = -F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t631 * t4650 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t97 * t13501 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t13181 * t13505 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t631 * t4653 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t97 * t13511 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t13516 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t4656 * t637 - F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t1327 * t3525 + F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1327 * t3529 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t105 * t13526 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t13202 * t13530 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t105 * t13534 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t13538;
    t13541
}
