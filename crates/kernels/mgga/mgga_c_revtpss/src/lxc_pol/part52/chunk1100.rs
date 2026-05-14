//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1100/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1100<F: Float>(t121661: F, t125336: F, t125260: F, t121656: F, t125268: F, t125279: F, t32597: F, t33621: F, t13272: F, t32596: F, t8623: F, t32589: F, t121629: F, t34177: F, t1493: F, t1925: F) -> (F, F, F, F, F, F, F, F, F) {
    let t128368 = t121661 * t125336;
    let t128371 = t121661 * t125260;
    let t128374 = t121656 * t125268;
    let t128377 = t121656 * t125279;
    let t128382 = t32597 * t33621;
    let t128385 = t13272 * t32596 * t8623;
    let t128394 = t13272 * t32589;
    let t128399 = t121629 * t34177;
    let t128401 = t1925 * t1493;
    (t128368, t128371, t128374, t128377, t128382, t128385, t128394, t128399, t128401)
}
