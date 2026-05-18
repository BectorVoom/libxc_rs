//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1114/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1114<F: Float>(t11064: F, t1468: F, t1711: F, t670: F, t7724: F, t7897: F, t8995: F, t3999: F, t8085: F, t198: F, t8034: F, t2718: F, t7997: F) -> (F, F, F, F, F, F, F) {
    let t106589 = t11064 * t1468;
    let t107923 = t11064 * t1711;
    let t108120 = t7724 * t670;
    let t109269 = t7897 * t8995;
    let t109731 = t3999 * t8085;
    let t110165 = t198 * t8034;
    let t110687 = t2718 * t7997;
    (t106589, t107923, t108120, t109269, t109731, t110165, t110687)
}
