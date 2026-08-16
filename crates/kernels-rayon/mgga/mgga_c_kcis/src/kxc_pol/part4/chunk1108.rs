//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1108/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1108(t13973: f64, t274: f64, t3589: f64, t4740: f64, t13908: f64, t13720: f64, t13726: f64, t13729: f64, t13735: f64, t13738: f64, t9700: f64, t9702: f64, t9708: f64, t9710: f64, t9712: f64) -> (f64, f64, f64) {
    let t13974 = t13973 * t274;
    let t13977 = t4740 * t3589;
    let t14001 = 0.22076e0_f64 * t13908;
    let t14002 = -0.20128333333333333334e0_f64 * t9700 - 0.11038e0_f64 * t9702 - 0.18396666666666666667e0_f64 * t9708 + 0.5519e-1_f64 * t9710 + 0.18396666666666666667e-1_f64 * t9712 - 0.20128333333333333333e0_f64 * t13729 - 0.33547222222222222222e0_f64 * t13720 - 0.80513333333333333333e0_f64 * t13726 + 0.60385e0_f64 * t13738 + 0.24154e1_f64 * t13735 - t14001;
    (t13974, t13977, t14002)
}
