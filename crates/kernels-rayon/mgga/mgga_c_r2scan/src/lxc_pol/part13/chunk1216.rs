//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1216/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1216(t40594: f64, t40595: f64, t4176: f64, t10935: f64, t2813: f64, t3446: f64, t11004: f64, t11523: f64, t6897: f64, t983: f64, t2330: f64, t3275: f64, t3276: f64) -> (f64, f64, f64, f64) {
    let t40598 = 45.0_f64 / 32.0_f64 * t40594 * t4176 * t40595;
    let t40603 = t3446 * t10935 * t2813;
    let t40604 = 0.19211284388664477842e-2_f64 * t40603;
    let t40606 = 5.0_f64 / 8.0_f64 * t11523 * t11004;
    let t40608 = t6897 * t983;
    let t40609 = t40608 * t2330;
    let t40612 = 5.0_f64 / 8.0_f64 * t3275 * t3276 * t40609;
    (t40598, t40604, t40606, t40612)
}
