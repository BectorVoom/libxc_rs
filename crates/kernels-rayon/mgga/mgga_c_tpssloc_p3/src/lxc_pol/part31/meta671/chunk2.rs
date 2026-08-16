//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2003/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2003(t1409: f64, t605: f64, t63: f64, t27961: f64, t84219: f64, t2032: f64, t26063: f64, t26070: f64, t26073: f64, t26076: f64, t26911: f64, t26945: f64, t27982: f64, t7035: f64, t7432: f64, t7435: f64, t7782: f64, t91907: f64, t96553: f64, t96556: f64, t96559: f64, t96562: f64) -> f64 {
    let t102227 = t605 * t1409 * t63;
    let t102248 = t84219 * t27961;
    let t102252 = -4.0_f64 / 3.0_f64 * t102227 * t96553 - 2.0_f64 / 3.0_f64 * t96556 * t2032 - 2.0_f64 / 3.0_f64 * t96559 * t2032 - 2.0_f64 / 3.0_f64 * t96562 * t2032 - 2.0_f64 / 3.0_f64 * t27982 * t7035 - 4.0_f64 / 3.0_f64 * t26070 * t7782 - 4.0_f64 / 3.0_f64 * t26073 * t7782 - 4.0_f64 / 3.0_f64 * t26076 * t7782 - 4.0_f64 / 3.0_f64 * t7435 * t26945 - 10.0_f64 / 3.0_f64 * t91907 * t7432 - 80.0_f64 / 3.0_f64 * t102248 - 10.0_f64 / 3.0_f64 * t26911 * t26063;
    t102252
}
