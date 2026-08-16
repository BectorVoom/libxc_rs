//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1168/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1168(t125456: f64, t125470: f64, t125472: f64, t125474: f64, t125483: f64, t125486: f64, t125488: f64, t125491: f64, t125495: f64, t125499: f64, t125505: f64, t125507: f64, t129312: f64, t129314: f64, t129316: f64, t129319: f64, t129322: f64, t4292: f64, t651: f64, t8964: f64) -> f64 {
    let t131226 = -2.0_f64 * t4292 * t651 * t8964 - t125456 - t125470 + t125472 - t125474 - t125483 + t125486 - t125488 - t125491 + t125495 - t125499 - t125505 - t125507 + 6.0_f64 * t129312 - 4.0_f64 * t129314 - 4.0_f64 * t129316 - 4.0_f64 * t129319 - 2.0_f64 * t129322;
    t131226
}
