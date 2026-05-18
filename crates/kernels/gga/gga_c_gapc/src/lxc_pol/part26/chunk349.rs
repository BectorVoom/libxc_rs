//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 349/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk349<F: Float>(t1565: F, t463: F, t4: F, t583: F, t1417: F, t1191: F, t120: F, t1476: F, t1484: F, t1488: F, t1492: F, t1495: F, t1498: F, t1503: F, t1505: F, t1511: F, t1514: F, t1555: F, t1559: F, t1562: F, t459: F, t469: F, t477: F, t523: F, t526: F) -> (F, F) {
    let t1566 = t463 * t1565;
    let t1567 = t583 * t4;
    let t1568 = t1417 * t1567;
    let t1571 = -F::new(0.33793761094153512599e-3) * t1484 * t523 + F::new(0.12360406057797588768e-3) * t1488 * t477 + F::new(0.1013812832824605378e-3) * t1492 * t523 + F::new(0.16221005325193686047e-3) * t1495 * t469 - F::new(0.20855578275249024918e-2) * t1498 * t459 + F::new(0.20855578275249024918e-2) * t1503 * t1505 + F::new(0.20855578275249024918e-2) * t526 * t1511 + F::new(0.1802559216762148362e-4) * t1476 * t1514 + F::new(0.10427789137624512459e-2) * t120 * t1555 - F::new(0.41201353525991962561e-5) * t1559 * t1562 - t1191 - F::new(0.12360406057797588768e-3) * t1566 * t1568;
    (t1567, t1571)
}
