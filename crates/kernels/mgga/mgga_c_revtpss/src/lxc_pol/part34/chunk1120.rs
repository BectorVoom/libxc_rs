//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1120/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1120<F: Float>(t1035: F, t29807: F, t29834: F, t7166: F, t1976: F, t6305: F, t3153: F, t6235: F, t4746: F, t7810: F, t7143: F, t106655: F, t994: F, t29833: F, t3056: F, t1646: F, t1651: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t107207 = t1035 * t29807;
    let t107212 = t29834 * t7166;
    let t107225 = t1976 * t6305;
    let t107226 = t107225 * t3153;
    let t107240 = t6235 * t1976;
    let t107283 = t4746 * t7810;
    let t107286 = t29834 * t7143;
    let t107358 = t4746 * t7143;
    let t107435 = t994 * t106655;
    let t107496 = t29833 * t3056 * t7143;
    let t107532 = t1646 * t1651;
    (t107207, t107212, t107226, t107240, t107283, t107286, t107358, t107435, t107496, t107532)
}
