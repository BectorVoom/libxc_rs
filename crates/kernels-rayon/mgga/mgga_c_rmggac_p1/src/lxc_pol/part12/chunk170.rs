//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 170/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk170(t326: f64, t559: f64, t305: f64, t344: f64, t349: f64, t551: f64, t558: f64) -> (f64, f64) {
    let t560 = t326 * t559;
    let t570 = 0.19957069503106347607e-1_f64 * t305 * t551 - 0.19957069503106347607e-1_f64 * t326 * t558 + 0.26552308210121162678e-3_f64 * t344 * t551 - 0.26552308210121162678e-3_f64 * t349 * t558;
    (t560, t570)
}
