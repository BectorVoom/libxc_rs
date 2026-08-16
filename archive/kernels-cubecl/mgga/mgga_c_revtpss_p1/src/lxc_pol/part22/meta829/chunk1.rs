//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2949/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2949<F: Float>(t2661: F, t3924: F, t3992: F, t5608: F, t1882: F, t4010: F, t9956: F, t13774: F, t5675: F, t9934: F, t1868: F, t4056: F) -> (F, F, F, F) {
    let t48453 = t2661 * t3992 * t5608 * t3924;
    let t48455 = t4010 * t1882;
    let t48458 = t2661 * t3992 * t48455 * t9956;
    let t48462 = t2661 * t9934 * t13774 * t5675;
    let t48466 = t1868 * t4056;
    (t48453, t48458, t48462, t48466)
}
