//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1092/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1092<F: Float>(t125559: F, t25082: F, t26405: F, t2014: F, t32113: F, t8108: F, t34021: F, t7235: F, t32626: F, t7937: F, t119578: F, t128235: F, t128236: F, t128240: F, t128242: F, t128244: F, t128245: F, t128251: F, t28718: F, t28932: F, t33913: F, t7489: F, t8568: F) -> (F,) {
    let t128254 = 3.0 * t25082 * t26405 * t125559;
    let t128256 = t2014 * t8108 * t32113;
    let t128260 = 3.0 * t7235 * t34021;
    let t128261 = t32626 * t7937;
    let t128262 = -3.0 * t119578 * t28718 + 3.0 * t28932 * t8568 + 3.0 * t33913 * t7489 - t128235 - t128236 + t128240 + t128242 - t128244 - t128245 - t128251 - t128254 - t128256 + t128260 - t128261;
    (t128262,)
}
