//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 928/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk928<F: Float>(t121110: F, t4021: F, t240: F, t31752: F, t545: F, t843: F, t32213: F, t125: F, t4075: F, t121035: F, t25875: F, t122: F, t72: F, t8578: F, t3916: F, t121072: F, t2453: F, t32217: F) -> (F, F, F, F, F, F, F, F, F) {
    let t121111 = t121110 * t4021;
    let t121116 = t31752 * t545 * t843 * t240;
    let t121117 = t121116 * t32213;
    let t121126 = t125 * t4075;
    let t121131 = t25875 * t121035;
    let t121133 = t8578 * t72 * t122;
    let t121134 = t121133 * t3916;
    let t121135 = t121131 * t121134;
    let t121139 = 0.3427046870806409921e-2 * t2453 * t32217 * t121072;
    (t121111, t121116, t121117, t121126, t121131, t121133, t121134, t121135, t121139)
}
