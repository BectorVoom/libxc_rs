//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 351/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk351(t1565: f64, t463: f64, t4: f64, t583: f64, t1417: f64, t1191: f64, t120: f64, t1476: f64, t1484: f64, t1488: f64, t1492: f64, t1495: f64, t1498: f64, t1503: f64, t1505: f64, t1511: f64, t1514: f64, t1555: f64, t1559: f64, t1562: f64, t459: f64, t469: f64, t477: f64, t523: f64, t526: f64) -> (f64, f64) {
    let t1566 = t463 * t1565;
    let t1567 = t583 * t4;
    let t1568 = t1417 * t1567;
    let t1571 = -0.33793761094153512599e-3_f64 * t1484 * t523 + 0.12360406057797588768e-3_f64 * t1488 * t477 + 0.1013812832824605378e-3_f64 * t1492 * t523 + 0.16221005325193686047e-3_f64 * t1495 * t469 - 0.20855578275249024918e-2_f64 * t1498 * t459 + 0.20855578275249024918e-2_f64 * t1503 * t1505 + 0.20855578275249024918e-2_f64 * t526 * t1511 + 0.1802559216762148362e-4_f64 * t1476 * t1514 + 0.10427789137624512459e-2_f64 * t120 * t1555 - 0.41201353525991962561e-5_f64 * t1559 * t1562 - t1191 - 0.12360406057797588768e-3_f64 * t1566 * t1568;
    (t1567, t1571)
}
