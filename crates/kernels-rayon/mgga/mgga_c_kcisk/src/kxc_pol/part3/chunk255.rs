//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 255/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk255(t1197: f64, t334: f64, t325: f64, t45: f64, t330: f64) -> (f64, f64, f64, f64) {
    let t1198 = t1197 * t334;
    let t1201 = t45 * t325;
    let t1202 = t330 * t330;
    let t1203 = 1.0_f64 / t1202;
    (t1198, t1201, t1202, t1203)
}
