//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1218/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1218(t34428: f64, t4254: f64, t651: f64, t7683: f64, t7741: f64, t118: f64, t125470: f64, t125472: f64, t125474: f64, t125475: f64, t125479: f64, t125483: f64, t125486: f64, t125488: f64, t127296: f64, t129308: f64, t129312: f64, t129314: f64) -> f64 {
    let t129316 = t4254 * t34428;
    let t129319 = t651 * t7683 * t7741;
    let t129321 = -t118 * (t129308 + t127296) - t125470 + t125472 - t125474 + t125475 + 2.0_f64 * t125479 - t125483 + 3.0_f64 * t129312 + t125486 - 2.0_f64 * t129314 - 2.0_f64 * t129316 - 2.0_f64 * t129319 - t125488;
    t129321
}
