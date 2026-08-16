//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1207/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1207<F: Float>(t107056: F, t107214: F, t20029: F, t20044: F, t20613: F, t27009: F, t6461: F, t7194: F, t7925: F, t7937: F, t84423: F, t97529: F, t97537: F, t97548: F, t97571: F) -> F {
    let t107731 = F::cast_from(0.46058153871750340221e0_f64) * t97529 - F::cast_from(0.3289868133696452873e-1_f64) * t107056 + t84423 + F::cast_from(12.0_f64) * t20029 * t7925 - F::cast_from(0.49348022005446793095e-1_f64) * t107214 + F::cast_from(0.23029076935875170111e0_f64) * t97537 - F::cast_from(0.23029076935875170111e0_f64) * t97548 - F::cast_from(3.0_f64) * t20044 * t7937 + F::cast_from(6.0_f64) * t7194 * t20613 - F::cast_from(3.0_f64) * t27009 * t6461 - F::cast_from(0.49348022005446793095e-1_f64) * t97571;
    t107731
}
