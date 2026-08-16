//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1049/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1049(t1017: f64, t16690: f64, t86: f64, t11418: f64, t556: f64, t3728: f64, t5673: f64, t4142: f64, t5776: f64, t11913: f64, t5650: f64, t1363: f64, t5623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16692 = t86 * t1017 * t16690;
    let t16693 = t556 * t11418;
    let t16719 = t3728 * t5673;
    let t16720 = 0.22109259259259259258e-2_f64 * t16719;
    let t16730 = t4142 * t5776;
    let t16731 = 0.22109259259259259258e-2_f64 * t16730;
    let t16732 = t11913 * t5650;
    let t16733 = 0.14739506172839506172e-2_f64 * t16732;
    let t16744 = t5623 * t1363;
    (t16692, t16693, t16719, t16720, t16730, t16731, t16732, t16733, t16744)
}
