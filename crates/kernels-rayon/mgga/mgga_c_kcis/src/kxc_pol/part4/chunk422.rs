//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 422/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk422(t1369: f64, t286: f64, t531: f64, t617: f64, t833: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1600 = t286 * t1369;
    let t1601 = t617 * t531;
    let t1602 = t1601 * t833;
    let t1603 = t1600 * t1602;
    let t1606 = t616 * t616;
    let t1607 = 1.0_f64 / t1606;
    (t1600, t1601, t1602, t1603, t1606, t1607)
}
