//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 901/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk901<F: Float>(t109: F, t104: F, t50: F, t656: F, t1449: F, t8184: F, t64: F, t8128: F, t8137: F, t8179: F, t8262: F) -> (F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t8266 = t656 * t50 * t104;
    let t8269 = t8184 * t1449;
    let t8273 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t8179 + t8128 * t8262 / F::cast_from(4.0_f64) + F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t64 * t8266 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8137 * t8269);
    (t8266, t8269, t8273)
}
