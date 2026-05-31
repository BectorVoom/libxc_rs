//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1218/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1218<F: Float>(t34428: F, t4254: F, t651: F, t7683: F, t7741: F, t118: F, t125470: F, t125472: F, t125474: F, t125475: F, t125479: F, t125483: F, t125486: F, t125488: F, t127296: F, t129308: F, t129312: F, t129314: F) -> F {
    let t129316 = t4254 * t34428;
    let t129319 = t651 * t7683 * t7741;
    let t129321 = -t118 * (t129308 + t127296) - t125470 + t125472 - t125474 + t125475 + F::cast_from(2.0_f64) * t125479 - t125483 + F::cast_from(3.0_f64) * t129312 + t125486 - F::cast_from(2.0_f64) * t129314 - F::cast_from(2.0_f64) * t129316 - F::cast_from(2.0_f64) * t129319 - t125488;
    t129321
}
