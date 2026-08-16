//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1303/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1303(t4142: f64, t7034: f64, t11814: f64, t7038: f64, t3728: f64, t6933: f64, t7113: f64, t833: f64, t1409: f64, t6281: f64, t1419: f64, t167: f64, t1951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21518 = t4142 * t7034;
    let t21520 = t11814 * t7038;
    let t21522 = t3728 * t6933;
    let t21524 = t7113 * t833;
    let t21527 = t1409 * t6281;
    let t21528 = t21527 * t1419;
    let t21531 = t1951 * t167;
    (t21518, t21520, t21522, t21524, t21528, t21531)
}
