//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 384/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk384(t1571: f64, t526: f64, t1480: f64, t1483: f64, t1486: f64, t1490: f64, t1492: f64, t1495: f64) -> (f64, f64) {
    let t1572 = t1571 * t526;
    let t1581 = -0.78438333333333333333e0_f64 * t1480 + 0.15687666666666666667e1_f64 * t1483 + 0.68863333333333333333e0_f64 * t1486 + 0.14025833333333333333e0_f64 * t1490 + 0.28051666666666666667e0_f64 * t1492 + 0.17365833333333333333e0_f64 * t1495;
    (t1572, t1581)
}
