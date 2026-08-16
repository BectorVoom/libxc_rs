//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 707/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk707(t4108: f64, t509: f64, t552: f64, t557: f64, t303: f64, t3245: f64, t558: f64, t1364: f64, t1387: f64, t3718: f64, t3725: f64, t3729: f64, t3731: f64, t3736: f64, t3740: f64, t3957: f64, t3961: f64, t3964: f64, t4013: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4109 = t509 * t4108;
    let t4110 = t4109 * t552;
    let t4111 = t4110 * t557;
    let t4112 = t303 * t4111;
    let t4114 = t3245 * t558;
    let t4115 = 0.55273148148148148147e-3_f64 * t4114;
    let t4116 = 0.66725e-1_f64 * t1364 * t3718 + 0.16581944444444444444e-2_f64 * t3725 - 0.33163888888888888888e-2_f64 * t3729 + 0.22109259259259259258e-2_f64 * t3731 - 0.49745833333333333332e-2_f64 * t3736 + 0.33163888888888888888e-2_f64 * t3740 - 0.24872916666666666666e-2_f64 * t3957 + 0.890445125e-2_f64 * t3961 * t3718 - 0.13345e0_f64 * t3964 * t1387 - 0.66725e-1_f64 * t1364 * t4013 + 0.24872916666666666666e-2_f64 * t4112 - t4115;
    (t4110, t4111, t4112, t4114, t4115, t4116)
}
