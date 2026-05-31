//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1168/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1168<F: Float>(t125456: F, t125470: F, t125472: F, t125474: F, t125483: F, t125486: F, t125488: F, t125491: F, t125495: F, t125499: F, t125505: F, t125507: F, t129312: F, t129314: F, t129316: F, t129319: F, t129322: F, t4292: F, t651: F, t8964: F) -> F {
    let t131226 = -F::cast_from(2.0_f64) * t4292 * t651 * t8964 - t125456 - t125470 + t125472 - t125474 - t125483 + t125486 - t125488 - t125491 + t125495 - t125499 - t125505 - t125507 + F::cast_from(6.0_f64) * t129312 - F::cast_from(4.0_f64) * t129314 - F::cast_from(4.0_f64) * t129316 - F::cast_from(4.0_f64) * t129319 - F::cast_from(2.0_f64) * t129322;
    t131226
}
