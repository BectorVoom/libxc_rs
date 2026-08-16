//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 833/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk833(t1580: f64, t27777: f64, t4419: f64, t8399: f64, t535: f64, t8336: f64, t2318: f64, t6497: f64, t3973: f64, t8331: f64, t1576: f64, t8308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27778 = t1580 * t27777;
    let t27790 = t4419 * t8399;
    let t27791 = t535 * t27790;
    let t27795 = t4419 * t8336;
    let t27796 = t535 * t27795;
    let t27810 = t2318 * t6497;
    let t27861 = t3973 * t8331;
    let t27862 = t1580 * t27861;
    let t27915 = t8308 * t1576;
    (t27778, t27791, t27796, t27810, t27862, t27915)
}
