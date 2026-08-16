//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1110/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1110(t41114: f64, t41128: f64, t118: f64, t338: f64, t35918: f64, t35922: f64, t35926: f64, t35937: f64, t41086: f64, t41095: f64, t41101: f64, t41106: f64, t41108: f64, t41120: f64, t43637: f64) -> f64 {
    let t44143 = 0.15965655602485078085e0_f64 * t41114;
    let t44145 = 0.3193131120497015617e0_f64 * t41128;
    let t44146 = -0.11974241701863808564e0_f64 * t41086 + 0.19957069503106347607e-1_f64 * t118 * t338 * t43637 - 0.11974241701863808564e0_f64 * t41095 - 0.35922725105591425692e0_f64 * t41101 + 0.71845450211182851384e0_f64 * t41106 + 0.17961362552795712846e0_f64 * t41108 + 0.95793933614910468511e0_f64 * t35918 + 0.1333427903096438929e0_f64 * t35922 + 0.53337116123857557162e0_f64 * t35926 + 0.36366215538993788974e-1_f64 * t35937 + t44143 + 0.95793933614910468511e0_f64 * t41120 - t44145;
    t44146
}
