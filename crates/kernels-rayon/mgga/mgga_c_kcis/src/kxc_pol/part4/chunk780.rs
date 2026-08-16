//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 780/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk780(t1102: f64, t1697: f64, t278: f64, t344: f64, t4597: f64, t4603: f64, t4608: f64, t4627: f64, t4630: f64, t4634: f64, t4639: f64, t4644: f64, t4672: f64, t4768: f64, t975: f64) -> f64 {
    let t4771 = 0.98556445e-3_f64 * t1102 * t4597 + 0.7391733375e-3_f64 * t1102 * t4603 - 0.1478346675e-2_f64 * t1102 * t4608 + 0.1478346675e-2_f64 * t344 * t4627 - 0.65704296666666666667e-3_f64 * t4630 - 0.65704296666666666667e-3_f64 * t1102 * t4634 - 0.1478346675e-2_f64 * t1102 * t4639 + 0.19711289e-2_f64 * t1102 * t4644 - 0.98556445e-3_f64 * t344 * t4672 - 4.0_f64 * t975 * t1697 - 4.0_f64 * t278 * t4768;
    t4771
}
