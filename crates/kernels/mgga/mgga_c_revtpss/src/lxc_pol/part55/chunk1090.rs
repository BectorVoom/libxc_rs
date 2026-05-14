//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1090/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1090<F: Float>(t34270: F, t7316: F, t2033: F, t25082: F, t26405: F, t5591: F, t125559: F, t2014: F, t32113: F, t8108: F, t34021: F, t7235: F, t32626: F, t7937: F, t122647: F, t28067: F) -> (F, F, F, F, F, F, F) {
    let t128245 = t34270 * t7316;
    let t128251 = 3.0 * t25082 * t26405 * t2033 * t5591;
    let t128254 = 3.0 * t25082 * t26405 * t125559;
    let t128256 = t2014 * t8108 * t32113;
    let t128260 = 3.0 * t7235 * t34021;
    let t128261 = t32626 * t7937;
    let t128266 = 3.0 * t122647 * t28067;
    (t128245, t128251, t128254, t128256, t128260, t128261, t128266)
}
