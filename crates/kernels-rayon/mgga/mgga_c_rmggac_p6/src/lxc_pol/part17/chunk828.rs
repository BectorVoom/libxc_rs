//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 828/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk828(t338: f64, t6444: f64, t39665: f64, t5259: f64, t38569: f64, t7782: f64, t321: f64, t8712: f64, t262: f64, t7785: f64, t8708: f64, t7844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40826 = t6444 * t338;
    let t40831 = t5259 * t39665;
    let t40832 = 0.15965655602485078085e0_f64 * t40831;
    let t40891 = t7782 * t38569;
    let t40897 = t8712 * t321;
    let t40898 = t262 * t40897;
    let t40899 = t7785 * t40898;
    let t40905 = t8708 * t321;
    let t40906 = t262 * t40905;
    let t40907 = t7844 * t40906;
    (t40826, t40832, t40891, t40897, t40898, t40899, t40905, t40906, t40907)
}
