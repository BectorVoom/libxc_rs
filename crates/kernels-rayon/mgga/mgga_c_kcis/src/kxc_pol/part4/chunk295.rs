//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 295/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk295(t1085: f64, t304: f64, t355: f64, t360: f64, t303: f64, t1017: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t1086 = t304 * t1085;
    let t1087 = t1086 * t355;
    let t1088 = t1087 * t360;
    let t1089 = t303 * t1088;
    let t1092 = t86 * t1017 * t304;
    (t1087, t1088, t1089, t1092)
}
