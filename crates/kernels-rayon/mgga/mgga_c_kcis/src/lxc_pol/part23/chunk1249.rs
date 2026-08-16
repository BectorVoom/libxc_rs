//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1249/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1249(t1464: f64, t1497: f64, t27387: f64, t52073: f64, t28505: f64, t3728: f64, t27376: f64, t28369: f64, t27459: f64, t28335: f64, t28480: f64, t7904: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98378 = t1464 * t27387 * t52073 * t1497;
    let t98380 = t3728 * t28505;
    let t98381 = 0.66327777777777777776e-2_f64 * t98380;
    let t98383 = 0.15445601851851851852e-3_f64 * t28369 * t27376;
    let t98387 = 0.15445601851851851852e-3_f64 * t27459 * t28335;
    let t98388 = t28480 * t7904;
    (t98378, t98380, t98381, t98383, t98387, t98388)
}
