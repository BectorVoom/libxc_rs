//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1333/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1333(t1437: f64, t16082: f64, t1330: f64, t16060: f64, t16078: f64, t16069: f64, t16055: f64, t3883: f64, t16065: f64, t5845: f64, t743: f64, t5848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17155 = t1437 * t16082;
    let t17158 = t1330 * t16060;
    let t17161 = t1437 * t16078;
    let t17164 = t1330 * t16069;
    let t17167 = t3883 * t16055;
    let t17170 = t1330 * t16065;
    let t17174 = 0.4705225e-4_f64 * t743 * t5845;
    let t17175 = t743 * t5848;
    (t17155, t17158, t17161, t17164, t17167, t17170, t17174, t17175)
}
