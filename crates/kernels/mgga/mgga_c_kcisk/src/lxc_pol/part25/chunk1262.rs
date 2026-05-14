//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1262/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1262<F: Float>(t32889: F, t9645: F, t32932: F, t9660: F, t112255: F, t9664: F, t112520: F, t9649: F, t112517: F, t33002: F, t17353: F, t33034: F, t33031: F, t33056: F, t3805: F, t9688: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112602 = t9645 * t32889;
    let t112604 = t32932 * t9660;
    let t112608 = t9664 * t112255;
    let t112610 = t9649 * t112520;
    let t112623 = t9649 * t112517;
    let t112637 = t33002 * t112520;
    let t112644 = t17353 * t33034;
    let t112645 = t33031 * t112644;
    let t112648 = t33056 * t112644;
    let t112661 = t3805 * t9688;
    (t112602, t112604, t112608, t112610, t112623, t112637, t112645, t112648, t112661)
}
