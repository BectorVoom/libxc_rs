//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 862/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk862(t38203: f64, t38204: f64, t38205: f64, t38206: f64, t38210: f64, t38211: f64, t9268: f64, t9269: f64, t9270: f64, t9271: f64, t9741: f64, t34544: f64, t34545: f64, t34548: f64, t34551: f64, t34554: f64, t7304: f64, t7308: f64, t7319: f64, t7340: f64, t8467: f64, t8470: f64) -> (f64, f64) {
    let t44518 = t38203 - t38204 - t38205 + t38206 - t38210 - t38211 + t9268 - t9269 + t9270 - t9271 - t9741;
    let t44526 = t34544 - t34545 - t7304 - t7308 + t34548 + 0.14408463291498358381e-2_f64 * t8467 - 0.20496175532535769484e-3_f64 * t8470 - t7319 + t34551 - t34554 - t7340;
    (t44518, t44526)
}
