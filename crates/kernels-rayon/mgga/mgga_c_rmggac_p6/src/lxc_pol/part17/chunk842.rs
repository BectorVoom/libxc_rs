//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 842/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk842(t41581: f64, t2286: f64, t7939: f64, t2019: f64, t2020: f64, t8858: f64, t8854: f64, t8850: f64, t22: f64, t235: f64, t26115: f64, t40921: f64, t8630: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41582 = 0.19863479950205658386e-4_f64 * t41581;
    let t41585 = t7939 * t2286;
    let t41604 = t2019 * t2020 * t8858;
    let t41605 = 0.30487649791575028314e-3_f64 * t41604;
    let t41613 = t2019 * t2020 * t8854;
    let t41614 = 0.30487649791575028314e-3_f64 * t41613;
    let t41619 = t2019 * t2020 * t8850;
    let t41620 = 0.30487649791575028314e-3_f64 * t41619;
    let t41634 = t235 * t26115 * t22;
    let t41637 = t8630 * t40921;
    (t41582, t41585, t41605, t41614, t41620, t41634, t41637)
}
