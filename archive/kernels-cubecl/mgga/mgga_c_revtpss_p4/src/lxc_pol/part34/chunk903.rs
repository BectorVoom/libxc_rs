//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 903/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk903<F: Float>(t1032: F, t6564: F, t1246: F, t127: F, t371: F, t6645: F, t1235: F, t6609: F, t3671: F, t1208: F, t6563: F, t225: F) -> (F, F, F, F, F) {
    let t20819 = t6564 * t1032;
    let t20820 = t20819 * t1246;
    let t20842 = t371 * t127 * t6645;
    let t20843 = t1235 * t20842;
    let t20846 = t371 * t127 * t6609;
    let t20847 = t3671 * t20846;
    let t20849 = t6563 * t1208;
    let t20850 = t20849 * t225;
    (t20820, t20843, t20847, t20849, t20850)
}
