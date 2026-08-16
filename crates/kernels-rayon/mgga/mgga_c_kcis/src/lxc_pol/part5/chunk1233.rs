//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1233/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1233(t1252: f64, t20648: f64, t11000: f64, t6774: f64, t1262: f64, t5329: f64, t5330: f64, t5336: f64, t11068: f64, t6758: f64, t1251: f64, t3500: f64, t6766: f64) -> (f64, f64, f64, f64, f64) {
    let t20649 = t1252 * t20648;
    let t20652 = t11000 * t6774;
    let t20653 = t20652 * t1262;
    let t20654 = t5329 * t20653;
    let t20657 = t5330 * t5336;
    let t20658 = t5329 * t20657;
    let t20661 = t11068 * t6758;
    let t20662 = t1251 * t20661;
    let t20666 = t3500 * t6766;
    (t20649, t20654, t20658, t20662, t20666)
}
