//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 481/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk481<F: Float>(t3985: F, t548: F, t1408: F, t820: F, t843: F, t1416: F, t1386: F, t240: F, t1398: F, t543: F, t550: F, t2661: F, t1384: F, t544: F) -> (F, F, F, F, F, F, F) {
    let t3987 = 0.56688979511669985553e-2 * t548 * t3985;
    let t3989 = t820 * t1408 * t843;
    let t3990 = t3989 * t1416;
    let t3992 = t1386 * t240;
    let t3994 = t550 * t1398 * t543;
    let t3995 = t3992 * t3994;
    let t3996 = t2661 * t3995;
    let t3999 = 1.0 / t1384 / t544;
    (t3987, t3989, t3990, t3992, t3994, t3996, t3999)
}
