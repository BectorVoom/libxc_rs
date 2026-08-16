//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1102/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1102(t13890: f64, t250: f64, t3106: f64, t4711: f64, t659: f64, t13720: f64, t13726: f64, t13729: f64, t13735: f64, t13738: f64, t9700: f64, t9702: f64, t9708: f64, t9710: f64, t9712: f64) -> (f64, f64, f64) {
    let t13892 = t250 * t3106 * t13890;
    let t13908 = t659 * t4711;
    let t13909 = 0.21908444444444444444e0_f64 * t13908;
    let t13910 = -0.19931111111111111111e0_f64 * t9700 - 0.10954222222222222222e0_f64 * t9702 - 0.18257037037037037037e0_f64 * t9708 + 0.54771111111111111111e-1_f64 * t9710 + 0.18257037037037037037e-1_f64 * t9712 - 0.19931111111111111111e0_f64 * t13729 - 0.33218518518518518518e0_f64 * t13720 - 0.79724444444444444445e0_f64 * t13726 + 0.59793333333333333334e0_f64 * t13738 + 0.23917333333333333334e1_f64 * t13735 - t13909;
    (t13892, t13908, t13910)
}
