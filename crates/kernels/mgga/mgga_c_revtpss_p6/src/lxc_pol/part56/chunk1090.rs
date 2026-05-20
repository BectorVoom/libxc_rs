//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1090/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1090<F: Float>(t2014: F, t33975: F, t7315: F, t22496: F, t25082: F, t36970: F, t8594: F, t9593: F, t28196: F, t28198: F, t32117: F, t7898: F) -> (F, F, F, F) {
    let t125488 = t2014 * t33975 * t7315;
    let t125491 = F::new(3.0) * t25082 * t36970 * t22496;
    let t125492 = t8594 * t9593;
    let t125495 = F::new(2.0) * t28196 * t125492 * t28198;
    let t125499 = t7898 * t32117;
    (t125488, t125491, t125495, t125499)
}
