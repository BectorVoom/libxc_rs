//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1066/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1066<F: Float>(t118: F, t125470: F, t125472: F, t125474: F, t125475: F, t125479: F, t125483: F, t125486: F, t125488: F, t127296: F, t129308: F, t129312: F, t129314: F, t129316: F, t129319: F, t28189: F, t8764: F) -> (F, F) {
    let t129321 = -t118 * (t129308 + t127296) - t125470 + t125472 - t125474 + t125475 + 2.0 * t125479 - t125483 + 3.0 * t129312 + t125486 - 2.0 * t129314 - 2.0 * t129316 - 2.0 * t129319 - t125488;
    let t129322 = t8764 * t28189;
    (t129321, t129322)
}
