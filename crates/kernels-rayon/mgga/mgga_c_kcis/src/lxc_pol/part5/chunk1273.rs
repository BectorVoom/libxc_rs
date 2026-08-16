//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1273/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1273(t1336: f64, t21170: f64, t16115: f64, t1907: f64, t5541: f64, t5574: f64, t11543: f64, t6954: f64, t3856: f64, t6986: f64, t653: f64, t6938: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21172 = 1.0_f64 * t21170 * t1336;
    let t21174 = 2.0_f64 * t16115 * t1907;
    let t21176 = 2.0_f64 * t5541 * t5574;
    let t21178 = 2.0_f64 * t11543 * t6954;
    let t21180 = 1.0_f64 * t3856 * t6986;
    let t21186 = t653 * t6938;
    (t21172, t21174, t21176, t21178, t21180, t21186)
}
