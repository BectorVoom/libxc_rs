//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1233/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1233(t56275: f64, t56287: f64, t59: f64, t85: f64, t22497: f64, t22562: f64, t22578: f64, t22657: f64, t22659: f64, t22661: f64, t22666: f64, t22675: f64, t22694: f64, t56062: f64, t56068: f64, t56263: f64) -> (f64, f64, f64) {
    let t56289 = (t56275 + t56287) * t59;
    let t56291 = 0.19751789702565206229e-1_f64 * t56289 * t85;
    let t56292 = -t22657 + t56062 - t22659 - t22661 - t56068 + t22666 + t56263 - t22675 - t22694 + t56291 - t22497 + t22562 + t22578;
    (t56289, t56291, t56292)
}
