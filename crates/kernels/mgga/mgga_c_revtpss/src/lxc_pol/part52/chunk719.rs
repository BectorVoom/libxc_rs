//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 719/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk719<F: Float>(t248: F, t8486: F, t3140: F, t3268: F, t1078: F, t1035: F, t1312: F, t8460: F, t196: F, t2011: F, t197: F) -> (F, F, F, F, F, F) {
    let t8487 = t8486 * t248;
    let t8515 = t3140 * t3268;
    let t8520 = t3140 * t1078;
    let t8521 = t8520 * t1035;
    let t8563 = t1312 * t8460;
    let t8564 = 2.0 * t8563;
    let t8567 = t2011 * t196;
    let t8568 = t8567 * t197;
    (t8487, t8515, t8521, t8564, t8567, t8568)
}
