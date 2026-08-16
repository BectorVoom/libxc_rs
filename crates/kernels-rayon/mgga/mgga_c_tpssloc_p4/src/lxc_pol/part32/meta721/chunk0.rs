//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2290/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2290(t29809: f64, t85639: f64, t1251: f64, t5392: f64, t1751: f64, t8034: f64, t29822: f64, t17635: f64, t17691: f64, t2128: f64, t24589: f64, t24601: f64, t27382: f64, t27388: f64, t27433: f64, t27434: f64, t27444: f64, t27549: f64, t27774: f64, t27820: f64, t4936: f64, t7287: f64, t8002: f64, t85652: f64, t94297: f64, t94354: f64, t94363: f64, t94365: f64, t94395: f64, t94458: f64) -> (f64, f64) {
    let t103130 = t85639 * t29809;
    let t103132 = t5392 * t1251;
    let t103143 = t8034 * t1751;
    let t103149 = t85639 * t29822;
    let t103164 = -0.10966227112321509577e-1_f64 * t24589 * t24601 * t27444 * t17691 + 0.18277045187202515961e-2_f64 * t103130 + 0.54831135561607547884e-2_f64 * t24589 * t24601 * t85652 * t103132 + 0.54831135561607547884e-2_f64 * t24589 * t94354 * t8002 + 0.16449340668482264365e-1_f64 * t2128 * t4936 * t27382 + 0.54831135561607547883e-2_f64 * t24589 * t103143 * t7287 - 0.14621636149762012769e-1_f64 * t94395 * t27434 + 0.18277045187202515961e-2_f64 * t103149 + 0.54831135561607547884e-2_f64 * t24589 * t94458 * t27433 + 0.36554090374405031923e-2_f64 * t27549 * t24601 * t27774 * t17635 + 0.54831135561607547884e-2_f64 * t24589 * t27820 * t27388 + t94363 + t94365 + 0.54831135561607547884e-2_f64 * t24589 * t94297 * t8002;
    (t103132, t103164)
}
