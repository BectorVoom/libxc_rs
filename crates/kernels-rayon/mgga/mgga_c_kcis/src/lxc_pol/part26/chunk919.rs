//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 919/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk919(t12159: f64, t7080: f64, t1380: f64, t613: f64, t5726: f64, t5732: f64, t3970: f64, t7072: f64, t1368: f64, t5691: f64, t5698: f64, t1650: f64, t1938: f64) -> (f64, f64, f64, f64, f64) {
    let t21468 = t12159 * t7080;
    let t21469 = t21468 * t1380;
    let t21470 = t613 * t21469;
    let t21473 = t5726 * t5732;
    let t21474 = t613 * t21473;
    let t21477 = t3970 * t7072;
    let t21478 = t1368 * t21477;
    let t21480 = t5691 * t5698;
    let t21484 = t1650 * t1938;
    (t21470, t21474, t21478, t21480, t21484)
}
