//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2385/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2385<F: Float>(t17883: F, t5351: F, t1269: F, t3766: F, t460: F, t1280: F, t17345: F, t1287: F, t17389: F, t17600: F, t1248: F, t5412: F) -> (F, F, F, F, F, F, F) {
    let t17884 = t5351 * t17883;
    let t17887 = t3766 * t1269;
    let t17888 = t460 * t17887;
    let t17893 = t1280 * t17345;
    let t17902 = t17389 * t1287;
    let t17905 = t17600 * t1287;
    let t17909 = t5412 * t1248 * t1287;
    (t17884, t17887, t17888, t17893, t17902, t17905, t17909)
}
