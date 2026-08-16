//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 920/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk920(t8557: f64, t8560: f64, t8564: f64, t8568: f64, t8572: f64, t8575: f64, t8579: f64, t8581: f64, t8583: f64, t8586: f64, t8591: f64, t10433: f64, t10445: f64, t10458: f64, t10470: f64, t10484: f64, t10496: f64, t10509: f64) -> f64 {
    let t10521 = 0.86898242813537603826e-4_f64 * t8557 + 0.43449121406768801913e-4_f64 * t8560 + 0.2534532082061513445e-4_f64 * t8564 - 0.86898242813537603826e-4_f64 * t8568 + 0.2534532082061513445e-4_f64 * t8572 - 0.24720812115595177536e-3_f64 * t8575 - 0.86898242813537603826e-4_f64 * t8579 + 0.5503555378190714909e-3_f64 * t8581 + 0.17319302560753675207e-3_f64 * t8583 - 0.20855578275249024918e-2_f64 * t8586 + 0.41711156550498049836e-2_f64 * t8591;
    let t10524 = t10433 + t10445 + t10458 + t10470 + t10484 + t10496 + t10509 + t10521;
    t10524
}
