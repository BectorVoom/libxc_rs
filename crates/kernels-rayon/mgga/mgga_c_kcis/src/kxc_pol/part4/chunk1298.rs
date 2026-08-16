//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1298/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1298(t1495: f64, t16653: f64, t1468: f64, t1464: f64, t2011: f64, t3722: f64, t4135: f64, t1395: f64, t3728: f64, t5877: f64, t1489: f64, t5627: f64) -> (f64, f64, f64, f64, f64) {
    let t16654 = t1495 * t16653;
    let t16655 = t1468 * t16654;
    let t16656 = t1464 * t16655;
    let t16658 = t2011 * t3722;
    let t16659 = t4135 * t16658;
    let t16660 = t1395 * t16659;
    let t16661 = t1464 * t16660;
    let t16663 = t3728 * t5877;
    let t16665 = t5627 * t1489;
    (t16656, t16658, t16661, t16663, t16665)
}
