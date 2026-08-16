//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1272/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1272(t12140: f64, t7064: f64, t1368: f64, t7053: f64, t990: f64, t3970: f64, t7076: f64, t3999: f64, t7086: f64, t1380: f64, t613: f64, t1315: f64, t6948: f64) -> (f64, f64, f64, f64, f64) {
    let t21154 = t12140 * t7064;
    let t21155 = t1368 * t21154;
    let t21157 = t7053 * t990;
    let t21162 = t3970 * t7076;
    let t21163 = t1368 * t21162;
    let t21165 = t3999 * t7086;
    let t21166 = t21165 * t1380;
    let t21167 = t613 * t21166;
    let t21170 = t6948 * t1315;
    (t21155, t21157, t21163, t21167, t21170)
}
