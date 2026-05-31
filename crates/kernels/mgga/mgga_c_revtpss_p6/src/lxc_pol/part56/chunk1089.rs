//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1089/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1089<F: Float>(t125453: F, t2014: F, t32119: F, t32297: F, t5542: F, t33657: F, t7235: F, t32114: F, t7898: F, t33652: F, t22496: F, t25082: F, t37110: F) -> (F, F, F, F, F, F) {
    let t125456 = F::cast_from(3.0_f64) * t2014 * t32119 * t125453;
    let t125470 = t2014 * t32297 * t5542;
    let t125472 = F::cast_from(3.0_f64) * t7235 * t33657;
    let t125474 = F::cast_from(2.0_f64) * t7898 * t32114;
    let t125483 = F::cast_from(2.0_f64) * t7235 * t33652;
    let t125486 = F::cast_from(6.0_f64) * t25082 * t37110 * t22496;
    (t125456, t125470, t125472, t125474, t125483, t125486)
}
