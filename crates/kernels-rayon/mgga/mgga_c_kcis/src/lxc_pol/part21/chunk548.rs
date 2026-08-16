//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 548/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk548(t1262: f64, t922: f64, t3515: f64, t1071: f64, t421: f64, t2630: f64, t1252: f64, t1253: f64, t2635: f64, t1258: f64, t420: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3516 = t922 * t1262;
    let t3517 = t3515 * t3516;
    let t3520 = t421 * t1071;
    let t3521 = t3520 * t2630;
    let t3522 = t1252 * t3521;
    let t3525 = t1253 * t2635;
    let t3526 = t1252 * t3525;
    let t3530 = 1.0_f64 / t1258 / t420;
    (t3516, t3517, t3521, t3522, t3525, t3526, t3530)
}
