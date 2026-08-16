//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2609/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2609<F: Float>(t1042: F, t20879: F, t20261: F, t20263: F, t20386: F, t20388: F, t20390: F, t20393: F, t20396: F, t20399: F, t20402: F, t20404: F, t20450: F, t20452: F, t20454: F, t20471: F, t20475: F, t20477: F, t20685: F) -> (F, F) {
    let t20880 = t1042 * t20879;
    let t20885 = -t20261 - t20263 - t20386 - t20388 - t20390 - t20393 + t20396 - t20399 - t20402 - t20404 + t20450 + t20452 + t20454 - t20471 + t20475 + t20477 + t20685;
    (t20880, t20885)
}
