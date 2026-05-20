//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1554/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1554<F: Float>(t1261: F, t24643: F, t3172: F, t24770: F, t3153: F, t17569: F, t20783: F, t1222: F, t140: F, t24816: F, t24820: F, t12915: F, t247: F, t24713: F, t5384: F) -> (F, F, F, F, F, F) {
    let t82827 = t1261 * t3172 * t24643;
    let t82859 = t24770 * t3153;
    let t82932 = t17569 * t20783;
    let t82980 = t1222 * t140 * t24816;
    let t82983 = t1222 * t140 * t24820;
    let t83014 = t5384 * t247 * t12915 * t24713;
    (t82827, t82859, t82932, t82980, t82983, t83014)
}
