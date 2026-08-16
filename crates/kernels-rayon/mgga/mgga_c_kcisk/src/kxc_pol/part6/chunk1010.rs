//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1010/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1010(t12975: f64, t19100: f64, t25590: f64, t25601: f64, t25609: f64, t30569: f64, t30572: f64, t30592: f64, t30595: f64, t30599: f64, t30603: f64, t1180: f64) -> (f64, f64) {
    let t30605 = -t12975 - 4.0_f64 / 9.0_f64 * t19100 + 2.0_f64 / 9.0_f64 * t25590 - 2.0_f64 / 3.0_f64 * t25601 + t25609 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t30592 + 4.0_f64 / 3.0_f64 * t30595 - 2.0_f64 / 3.0_f64 * t30569 - 2.0_f64 * t30599 + 2.0_f64 * t30572 - t30603 / 3.0_f64;
    let t30606 = t1180 * t30605;
    (t30605, t30606)
}
