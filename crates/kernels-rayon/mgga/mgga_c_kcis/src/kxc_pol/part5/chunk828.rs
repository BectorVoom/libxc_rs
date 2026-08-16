//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 828/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk828(t3200: f64, t6626: f64, t1022: f64, t6330: f64, t1021: f64, t1020: f64, t1710: f64, t4787: f64, t4981: f64, t5003: f64, t5017: f64, t5023: f64, t6558: f64, t6561: f64, t6564: f64, t6616: f64, t6622: f64) -> (f64, f64, f64, f64, f64) {
    let t6627 = t3200 * t6626;
    let t6629 = t1022 * t6330;
    let t6630 = t1021 * t6629;
    let t6631 = t1020 * t6630;
    let t6633 = -0.13345e0_f64 * t4981 * t1710 - 0.33163888888888888888e-2_f64 * t5017 + 0.22109259259259259258e-2_f64 * t5023 + 0.33163888888888888888e-2_f64 * t4787 + 0.16581944444444444444e-2_f64 * t6558 - 0.49745833333333333332e-2_f64 * t6561 + 0.33163888888888888888e-2_f64 * t6564 - 0.24872916666666666666e-2_f64 * t6616 + 0.22109259259259259258e-2_f64 * t5003 - 0.33163888888888888888e-2_f64 * t6622 + 0.22109259259259259258e-2_f64 * t6627 - 0.33163888888888888888e-2_f64 * t6631;
    (t6627, t6629, t6630, t6631, t6633)
}
