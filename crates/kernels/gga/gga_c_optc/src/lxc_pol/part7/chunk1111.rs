//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1111/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1111<F: Float>(t2270: F, t3813: F, t7899: F, t889: F, t2613: F, t2620: F, t24985: F, t329: F, t7426: F, t7448: F, t10953: F, t24088: F, t25194: F, t25197: F, t25200: F, t25202: F, t25208: F, t25215: F, t25220: F, t2623: F, t2630: F, t2635: F, t2640: F, t2643: F, t2655: F, t300: F, t301: F, t314: F, t3821: F, t6541: F, t7355: F, t7376: F, t7407: F, t7410: F, t7449: F, t7451: F, t7485: F, t7488: F, t7495: F, t7838: F, t875: F) -> (F,) {
    let t25227 = t3813 * t2270;
    let t25237 = t7899 * t889;
    let t25239 = t2613 * t2620;
    let t25243 = t329 * t24985;
    let t25246 = t7426 * t7448;
    let t25249 = -t25194 / 216.0 - t25197 / 162.0 - t25200 / 27.0 - 4.0 / 81.0 * t25202 + 8.0 / 27.0 * t2623 * t7407 + 11.0 / 54.0 * t7410 * t2630 - t25208 / 27.0 + 22.0 / 81.0 * t7410 * t2635 - 0.25244669503346875858e1 * t7488 * t7485 + 0.18933502127510156893e0 * t25215 + 0.31555836879183594821e0 * t25220 + 0.94667510637550784468e-1 * t2640 * t3821 * t6541 * t875 * t2643 + 0.36629113921839320675e2 * t7449 * t7451 * t25227 + 0.23181763972770020945e0 * t10953 * t7355 + 0.56076257758205259001e1 * t300 * t301 * t24088 * t314 - 0.1794440248262568288e1 * t25237 - 0.12234819874517511055e0 * t25239 - 0.75734008510040627576e0 * t2655 * t7838 - 0.3779380353163141838e5 * t25243 * t7376 - 0.39071054849961942054e3 * t25246 * t7495;
    (t25249,)
}
