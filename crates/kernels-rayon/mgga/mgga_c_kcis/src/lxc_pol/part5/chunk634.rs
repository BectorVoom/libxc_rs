//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 634/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk634(t556: f64, t622: f64, t3793: f64, t1559: f64, t1563: f64, t1562: f64, t597: f64, t592: f64, t3879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4301 = 1.0_f64 / t556;
    let t4313 = t622 * t622;
    let t4314 = 1.0_f64 / t4313;
    let t4318 = 0.22831111111111111111e-1_f64 * t3793;
    let t4326 = t1559 * t1563;
    let t4329 = t1562 * t597;
    let t4330 = 1.0_f64 / t4329;
    let t4331 = t592 * t4330;
    let t4338 = 0.68863333333333333333e0_f64 * t3793;
    let t4345 = 0.17365833333333333333e0_f64 * t3879;
    let t4354 = t1562 * t1562;
    let t4355 = 1.0_f64 / t4354;
    let t4356 = t592 * t4355;
    (t4301, t4313, t4314, t4318, t4326, t4330, t4331, t4338, t4345, t4354, t4355, t4356)
}
