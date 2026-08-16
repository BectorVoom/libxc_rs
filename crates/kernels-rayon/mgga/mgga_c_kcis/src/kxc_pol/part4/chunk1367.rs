//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1367/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1367(t1517: f64, t2645: f64, t5987: f64, t1979: f64, t3754: f64, t2642: f64, t4219: f64, t2018: f64, t456: f64, t3820: f64, t562: f64, t143: f64, t16349: f64) -> (f64, f64, f64, f64, f64) {
    let t17605 = t1517 * t5987 * t2645;
    let t17608 = t1979 * t3754;
    let t17610 = t4219 * t17608 * t2642;
    let t17613 = t2018 * t456;
    let t17627 = t562 * t3820;
    let t17630 = t16349 * t143;
    (t17605, t17610, t17613, t17627, t17630)
}
