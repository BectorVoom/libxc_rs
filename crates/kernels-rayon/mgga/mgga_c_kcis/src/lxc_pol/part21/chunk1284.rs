//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1284/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1284(t283: f64, t9588: f64, t1092: f64, t1800: f64, t3228: f64, t27764: f64, t3226: f64, t982: f64, t5025: f64, t26762: f64, t1009: f64, t4972: f64) -> (f64, f64, f64, f64, f64) {
    let t95655 = t9588 * t283;
    let t95658 = t1092 * t95655 * t1800 * t3228;
    let t95662 = t1092 * t3226 * t982 * t27764;
    let t95664 = t5025 * t283;
    let t95666 = t1092 * t95664 * t26762;
    let t95670 = t1009 * t4972;
    (t95658, t95662, t95664, t95666, t95670)
}
