//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2275/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2275(t1751: f64, t7319: f64, t1240: f64, t5088: f64, t1089: f64, t3597: f64, t1090: f64, t12648: f64, t1409: f64, t14165: f64, t24589: f64, t24601: f64, t24883: f64, t24887: f64, t27381: f64, t27444: f64, t27445: f64, t27549: f64, t27774: f64, t27775: f64, t27820: f64, t3248: f64, t3252: f64, t3599: f64, t7287: f64, t8002: f64, t85640: f64, t85648: f64, t86415: f64) -> (f64, f64) {
    let t94297 = t7319 * t1751;
    let t94319 = t1240 * t5088;
    let t94332 = t3597 * t1089;
    let t94341 = 0.73108180748810063846e-2_f64 * t27549 * t86415 * t27775 + 0.54831135561607547884e-2_f64 * t24589 * t94297 * t7287 + 0.27415567780803773942e-2_f64 * t24589 * t27820 * t24883 + 0.54831135561607547884e-2_f64 * t24589 * t27820 * t24887 + 0.27415567780803773942e-2_f64 * t24589 * t24601 * t27381 * t3252 + 0.54831135561607547884e-2_f64 * t24589 * t24601 * t27381 * t3248 + 0.27415567780803773942e-2_f64 * t24589 * t85648 * t8002 + 0.18277045187202515961e-2_f64 * t85640 + 0.54831135561607547884e-2_f64 * t24589 * t24601 * t94319 * t1090 - 0.54831135561607547884e-2_f64 * t24589 * t24601 * t27444 * t12648 - 0.16449340668482264365e-1_f64 * t24589 * t24601 * t27774 * t14165 - 0.54831135561607547884e-2_f64 * t24589 * t24601 * t94332 * t1409 * t3599 - 0.10966227112321509577e-1_f64 * t24589 * t86415 * t27445;
    (t94319, t94341)
}
