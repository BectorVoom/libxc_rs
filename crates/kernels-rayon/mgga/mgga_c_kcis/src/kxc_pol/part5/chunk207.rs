//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 207/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk207(t651: f64, t653: f64, t657: f64, t659: f64, t31: f64) -> (f64, f64) {
    let t661 = -0.632975e0_f64 * t651 - 0.29896666666666666667e0_f64 * t653 - 0.1023875e0_f64 * t657 - 0.82156666666666666667e-1_f64 * t659;
    let t662 = 1.0_f64 / t31;
    (t661, t662)
}
