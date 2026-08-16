//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1306/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1306(t1409: f64, t21585: f64, t5526: f64, t5792: f64, t17057: f64, t1961: f64, t7119: f64, t833: f64, t6284: f64, t1419: f64, t7123: f64, t11939: f64, t7122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21586 = t1409 * t21585;
    let t21594 = t5792 * t5526;
    let t21597 = t17057 * t1961;
    let t21600 = t7119 * t833;
    let t21603 = t1409 * t6284;
    let t21604 = t21603 * t1419;
    let t21607 = t7123 * t833;
    let t21610 = t11939 * t7122;
    (t21586, t21594, t21597, t21600, t21604, t21607, t21610)
}
