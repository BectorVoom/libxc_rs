//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 986/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk986<F: Float>(t121210: F, t2453: F, t8705: F, t25304: F, t32237: F, t121142: F, t596: F, t8571: F, t32186: F, t786: F, t32216: F, t4075: F, t122: F, t32219: F, t3916: F, t119833: F, t121245: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t121272 = t2453 * t8705 * t121210;
    let t121273 = 0.3718732920905101082e-5 * t121272;
    let t121275 = t25304 * t8705 * t121210;
    let t121276 = 0.19835721400107809171e-4 * t121275;
    let t121285 = t2453 * t32237;
    let t121287 = 0.95199562775170587692e-3 * t121285 * t121142;
    let t121305 = t8571 * t596;
    let t121307 = t786 * t121305 * t32186;
    let t121308 = 0.20077843028252776532e-3 * t121307;
    let t121309 = t32216 * t4075;
    let t121310 = t786 * t121309;
    let t121312 = t32219 * t122 * t3916;
    let t121313 = t121310 * t121312;
    let t121326 = t119833 * t121245;
    (t121273, t121276, t121287, t121305, t121308, t121309, t121310, t121312, t121313, t121326)
}
