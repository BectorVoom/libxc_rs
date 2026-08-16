//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1317/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1317(t1464: f64, t21801: f64, t1497: f64, t6927: f64, t1495: f64, t4123: f64, t17298: f64, t5656: f64, t1981: f64, t3751: f64, t1017: f64, t86: f64) -> (f64, f64, f64, f64, f64) {
    let t21802 = t1464 * t21801;
    let t21804 = t6927 * t1497;
    let t21805 = t1495 * t21804;
    let t21806 = t4123 * t21805;
    let t21807 = t1464 * t21806;
    let t21811 = t17298 * t5656;
    let t21813 = t3751 * t1981;
    let t21815 = t86 * t1017 * t21813;
    (t21802, t21804, t21807, t21811, t21815)
}
