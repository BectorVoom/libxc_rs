//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1457/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1457(t213: f64, t5527: f64, t221: f64, t776: f64, t4119: f64, t4128: f64, t12986: f64, t13002: f64, t13005: f64, t13010: f64, t16769: f64, t4127: f64, t9526: f64, t9540: f64, t9542: f64, t9547: f64, t9572: f64) -> f64 {
    let t16771 = t213 * t5527;
    let t16773 = t221 * t16771 * t776;
    let t16777 = t221 * t4128 * t4119;
    let t16781 = 0.16666666666666666666e-2_f64 * t9526 - t9540 - 0.12962962962962962963e-1_f64 * t9542 - 0.52777777777777777776e-2_f64 * t9547 + 0.33333333333333333332e-2_f64 * t12986 - t13002 - t9572 - 0.11666666666666666666e-1_f64 * t16769 - 0.19999999999999999999e-1_f64 * t13005 * t16773 + 0.99999999999999999996e-2_f64 * t4127 * t16777 - 0.25925925925925925925e-1_f64 * t13010;
    t16781
}
