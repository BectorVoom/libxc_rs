//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 839/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk839(t34884: f64, t9118: f64, t2283: f64, t34881: f64, t2286: f64, t7939: f64, t2019: f64, t2020: f64, t8858: f64, t8854: f64, t8850: f64, t22: f64, t235: f64, t26115: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    let t41585 = t7939 * t2286;
    let t41604 = t2019 * t2020 * t8858;
    let t41613 = t2019 * t2020 * t8854;
    let t41619 = t2019 * t2020 * t8850;
    let t41634 = t235 * t26115 * t22;
    (t41579, t41581, t41585, t41604, t41613, t41619, t41634)
}
