//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1223/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1223<F: Float>(t7426: F, t7448: F, t10953: F, t24088: F, t25194: F, t25197: F, t25200: F, t25202: F, t25208: F, t25215: F, t25220: F, t25227: F, t25237: F, t25239: F, t25243: F, t2623: F, t2630: F, t2635: F, t2640: F, t2643: F, t2655: F, t300: F, t301: F, t314: F, t3821: F, t6541: F, t7355: F, t7376: F, t7407: F, t7410: F, t7449: F, t7451: F, t7485: F, t7488: F, t7495: F, t7838: F, t875: F) -> F {
    let t25246 = t7426 * t7448;
    let t25249 = -t25194 / F::cast_from(216.0_f64) - t25197 / F::cast_from(162.0_f64) - t25200 / F::cast_from(27.0_f64) - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t25202 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2623 * t7407 + F::cast_from(11.0_f64) / F::cast_from(54.0_f64) * t7410 * t2630 - t25208 / F::cast_from(27.0_f64) + F::cast_from(22.0_f64) / F::cast_from(81.0_f64) * t7410 * t2635 - F::cast_from(0.25244669503346875858e1_f64) * t7488 * t7485 + F::cast_from(0.18933502127510156893e0_f64) * t25215 + F::cast_from(0.31555836879183594821e0_f64) * t25220 + F::cast_from(0.94667510637550784468e-1_f64) * t2640 * t3821 * t6541 * t875 * t2643 + F::cast_from(0.36629113921839320675e2_f64) * t7449 * t7451 * t25227 + F::cast_from(0.23181763972770020945e0_f64) * t10953 * t7355 + F::cast_from(0.56076257758205259001e1_f64) * t300 * t301 * t24088 * t314 - F::cast_from(0.1794440248262568288e1_f64) * t25237 - F::cast_from(0.12234819874517511055e0_f64) * t25239 - F::cast_from(0.75734008510040627576e0_f64) * t2655 * t7838 - F::cast_from(0.3779380353163141838e5_f64) * t25243 * t7376 - F::cast_from(0.39071054849961942054e3_f64) * t25246 * t7495;
    t25249
}
