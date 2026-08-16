//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1004/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1004(t321: f64, t35918: f64, t35922: f64, t35926: f64, t35937: f64, t41101: f64, t41106: f64, t41108: f64, t41115: f64, t41116: f64, t41120: f64, t41122: f64, t4669: f64, t5259: f64, t833: f64, t848: f64, t876: f64, t8936: f64, t8975: f64) -> f64 {
    let t41126 = -0.17961362552795712846e0_f64 * t4669 * t8975 * t848 - 0.17961362552795712846e0_f64 * t41101 + 0.11974241701863808564e0_f64 * t5259 * t8975 * t833 + 0.35922725105591425692e0_f64 * t41106 + 0.8980681276397856423e-1_f64 * t41108 + 0.47896966807455234256e0_f64 * t35918 + 0.66671395154821946448e-1_f64 * t35922 + 0.2666855806192877858e0_f64 * t35926 + 0.18183107769496894486e-1_f64 * t35937 + t41115 - 0.47896966807455234256e0_f64 * t41116 * t8936 * t876 + 0.47896966807455234256e0_f64 * t41120 - 0.35922725105591425692e0_f64 * t4669 * t41122 * t321;
    t41126
}
