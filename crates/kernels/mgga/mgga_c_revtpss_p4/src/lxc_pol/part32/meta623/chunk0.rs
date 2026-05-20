//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1966/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1966<F: Float>(t29547: F, t644: F, t77: F, t1927: F, t5872: F, t2247: F, t5826: F, t196: F, t197: F, t22525: F, t1448: F, t6781: F) -> (F, F, F, F, F) {
    let t108983 = t77 * t29547 * t644;
    let t108986 = t1927 * t5872;
    let t108990 = t2247 * t5826;
    let t109077 = t22525 * t196 * t197;
    let t109096 = t6781 * t1448;
    (t108983, t108986, t108990, t109077, t109096)
}
