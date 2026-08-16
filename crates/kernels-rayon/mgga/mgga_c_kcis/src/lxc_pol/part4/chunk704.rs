//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 704/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk704(t1452: f64, t743: f64, t1451: f64, t3805: f64, t1430: f64, t3797: f64, t1431: f64, t733: f64, t542: f64, t1438: f64, t738: f64, t113: f64, t3754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4073 = t743 * t1452;
    let t4075 = t1451 * t3805;
    let t4078 = t1430 * t3797;
    let t4081 = t733 * t1431;
    let t4083 = t1430 * t3805;
    let t4086 = t542 * t3797;
    let t4089 = t738 * t1438;
    let t4093 = t113 * t3754;
    (t4073, t4075, t4078, t4081, t4083, t4086, t4089, t4093)
}
