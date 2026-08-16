//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1290/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1290(t1482: f64, t16533: f64, t542: f64, t1477: f64, t16194: f64, t3255: f64, t5432: f64, t5436: f64, t5442: f64, t1419: f64, t5808: f64, t5498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16534 = t1482 * t16533;
    let t16535 = t542 * t16534;
    let t16538 = t1477 * t16194;
    let t16539 = t542 * t16538;
    let t16543 = 0.13140859333333333334e-2_f64 * t3255 * t5432;
    let t16545 = 0.8760572888888888889e-3_f64 * t3255 * t5436;
    let t16547 = 0.17521145777777777778e-2_f64 * t3255 * t5442;
    let t16548 = t5808 * t1419;
    let t16549 = t5498 * t16548;
    (t16534, t16535, t16538, t16539, t16543, t16545, t16547, t16548, t16549)
}
