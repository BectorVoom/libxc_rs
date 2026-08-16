//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1398/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1398<F: Float>(t127: F, t3661: F, t371: F, t1235: F, t12640: F, t225: F, t12657: F, t480: F, t3667: F, t3678: F, t1236: F, t676: F) -> (F, F, F, F, F, F) {
    let t12963 = t371 * t127 * t3661;
    let t12964 = t1235 * t12963;
    let t12966 = t12640 * t225;
    let t12975 = t12657 * t225;
    let t12976 = t12975 * t480;
    let t12979 = t3667 * t3678;
    let t12984 = t371 * t676 * t1236;
    (t12964, t12966, t12975, t12976, t12979, t12984)
}
