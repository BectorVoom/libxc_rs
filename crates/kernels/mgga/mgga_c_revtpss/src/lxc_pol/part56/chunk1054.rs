//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1054/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1054<F: Float>(t121106: F, t25875: F, t4021: F, t32268: F, t240: F, t31752: F, t545: F, t843: F, t32213: F, t125: F, t4075: F, t121035: F) -> (F, F, F, F, F, F, F, F) {
    let t121107 = t25875 * t121106;
    let t121108 = t121107 * t4021;
    let t121110 = t32268 * t121106;
    let t121111 = t121110 * t4021;
    let t121116 = t31752 * t545 * t843 * t240;
    let t121117 = t121116 * t32213;
    let t121126 = t125 * t4075;
    let t121131 = t25875 * t121035;
    (t121107, t121108, t121110, t121111, t121116, t121117, t121126, t121131)
}
