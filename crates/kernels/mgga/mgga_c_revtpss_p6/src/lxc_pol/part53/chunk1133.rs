//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1133/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1133<F: Float>(t125453: F, t2014: F, t32119: F, t28043: F, t6985: F, t28182: F, t8568: F, t32297: F, t5542: F, t33657: F, t7235: F, t32114: F, t7898: F) -> (F, F, F, F, F, F) {
    let t125456 = F::cast_from(3.0_f64) * t2014 * t32119 * t125453;
    let t125459 = t6985 * t28043;
    let t125467 = t8568 * t28182;
    let t125470 = t2014 * t32297 * t5542;
    let t125472 = F::cast_from(3.0_f64) * t7235 * t33657;
    let t125474 = F::cast_from(2.0_f64) * t7898 * t32114;
    (t125456, t125459, t125467, t125470, t125472, t125474)
}
