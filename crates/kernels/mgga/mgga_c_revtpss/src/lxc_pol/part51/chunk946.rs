//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 946/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk946<F: Float>(t121165: F, t9794: F, t2453: F, t8571: F, t240: F, t27: F, t545: F, t119833: F, t124: F, t1426: F, t13847: F, t1444: F, t25898: F, t786: F, t8578: F, t4104: F) -> (F, F, F, F, F, F, F, F, F) {
    let t121166 = t9794 * t121165;
    let t121167 = t2453 * t8571 * t121166;
    let t121168 = 0.13386901839087538753e-4 * t121167;
    let t121173 = t545 * t27 * t240;
    let t121174 = t119833 * t121173;
    let t121175 = t124 * t1426;
    let t121177 = t13847 * t121175 * t1444;
    let t121178 = t121174 * t121177;
    let t121181 = t786 * t8578 * t25898;
    let t121182 = t121181 * t4104;
    (t121166, t121168, t121173, t121174, t121175, t121177, t121178, t121181, t121182)
}
