//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1086/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1086(t18574: f64, t4579: f64, t3269: f64, t6334: f64, t934: f64, t3255: f64, t6574: f64, t6578: f64, t1098: f64, t6606: f64, t6570: f64, t6582: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18575 = t4579 * t18574;
    let t18579 = t3269 * t6334 * t934;
    let t18582 = t3255 * t6574;
    let t18584 = t3255 * t6578;
    let t18586 = t1098 * t6606;
    let t18588 = t3255 * t6570;
    let t18590 = t3255 * t6582;
    (t18575, t18579, t18582, t18584, t18586, t18588, t18590)
}
