//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1223/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1223(t7426: f64, t7448: f64, t10953: f64, t24088: f64, t25194: f64, t25197: f64, t25200: f64, t25202: f64, t25208: f64, t25215: f64, t25220: f64, t25227: f64, t25237: f64, t25239: f64, t25243: f64, t2623: f64, t2630: f64, t2635: f64, t2640: f64, t2643: f64, t2655: f64, t300: f64, t301: f64, t314: f64, t3821: f64, t6541: f64, t7355: f64, t7376: f64, t7407: f64, t7410: f64, t7449: f64, t7451: f64, t7485: f64, t7488: f64, t7495: f64, t7838: f64, t875: f64) -> f64 {
    let t25246 = t7426 * t7448;
    let t25249 = -t25194 / 216.0_f64 - t25197 / 162.0_f64 - t25200 / 27.0_f64 - 4.0_f64 / 81.0_f64 * t25202 + 8.0_f64 / 27.0_f64 * t2623 * t7407 + 11.0_f64 / 54.0_f64 * t7410 * t2630 - t25208 / 27.0_f64 + 22.0_f64 / 81.0_f64 * t7410 * t2635 - 0.25244669503346875858e1_f64 * t7488 * t7485 + 0.18933502127510156893e0_f64 * t25215 + 0.31555836879183594821e0_f64 * t25220 + 0.94667510637550784468e-1_f64 * t2640 * t3821 * t6541 * t875 * t2643 + 0.36629113921839320675e2_f64 * t7449 * t7451 * t25227 + 0.23181763972770020945e0_f64 * t10953 * t7355 + 0.56076257758205259001e1_f64 * t300 * t301 * t24088 * t314 - 0.1794440248262568288e1_f64 * t25237 - 0.12234819874517511055e0_f64 * t25239 - 0.75734008510040627576e0_f64 * t2655 * t7838 - 0.3779380353163141838e5_f64 * t25243 * t7376 - 0.39071054849961942054e3_f64 * t25246 * t7495;
    t25249
}
