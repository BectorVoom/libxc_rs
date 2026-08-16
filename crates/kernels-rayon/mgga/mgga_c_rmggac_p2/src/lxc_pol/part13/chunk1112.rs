//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1112/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1112(t41438: f64, t1652: f64, t698: f64, t2227: f64, t551: f64, t1614: f64, t118: f64, t2463: f64, t27176: f64, t326: f64, t333: f64, t352: f64, t41116: f64, t41458: f64, t43698: f64, t43854: f64, t43981: f64, t44157: f64, t5148: f64, t5155: f64, t5245: f64, t5266: f64, t833: f64, t848: f64, t876: f64, t9540: f64, t9551: f64) -> (f64, f64, f64, f64) {
    let t44169 = 0.3193131120497015617e0_f64 * t41438;
    let t44183 = t698 * t1652;
    let t44187 = t2227 * t551;
    let t44194 = t698 * t1614;
    let t44203 = t44169 - 0.95793933614910468512e0_f64 * t27176 * t43981 - 0.11974241701863808564e0_f64 * t5148 * t9551 * t833 + 0.23948483403727617128e0_f64 * t5266 * t44157 * t333 - 0.47896966807455234256e0_f64 * t41116 * t9551 * t876 - 0.39914139006212695214e-1_f64 * t118 * t43698 + 0.23948483403727617128e0_f64 * t5266 * t44183 * t333 - 0.23948483403727617128e0_f64 * t5148 * t44187 * t352 - 0.59871208509319042821e-1_f64 * t326 * t43854 - 0.17961362552795712846e0_f64 * t41458 + 0.47896966807455234256e0_f64 * t5155 * t44194 * t333 + 0.23948483403727617128e0_f64 * t5155 * t9540 * t848 + 0.11974241701863808564e0_f64 * t5245 * t2463;
    (t44183, t44187, t44194, t44203)
}
