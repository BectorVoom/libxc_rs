//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 788/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk788(t1717: f64, t331: f64, t829: f64, t1035: f64, t1646: f64, t1045: f64, t167: f64, t313: f64, t1027: f64, t1728: f64, t1727: f64, t3073: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4831 = t331 * t1717;
    let t4833 = t1717 * t829;
    let t4836 = t1035 * t1646;
    let t4837 = t4836 * t1045;
    let t4840 = t313 * t167;
    let t4843 = t1027 * t1728;
    let t4845 = t1728 * t829;
    let t4848 = t3073 * t1727;
    (t4831, t4833, t4836, t4837, t4840, t4843, t4845, t4848)
}
