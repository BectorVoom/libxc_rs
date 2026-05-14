//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 769/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk769<F: Float>(t159: F, t853: F, t216: F, t2729: F, t794: F, t10111: F, t22: F, t870: F, t10115: F, t253: F, t10867: F, t251: F, t2778: F, t9303: F, t871: F, t9292: F) -> (F, F, F, F, F, F, F) {
    let t10899 = t159 * t853;
    let t10900 = t216 * t10899;
    let t10905 = t794 * t2729;
    let t10939 = 0.19637199382202157274e-3 * t10111 * t870 * t22;
    let t10948 = 0.11044544084478153697e-3 * t10115 * t253;
    let t10952 = t10867 * t251;
    let t10969 = 0.26019841438354088051e-2 * t9303 * t2778;
    let t10971 = 0.17073386770573548589e-1 * t9292 * t871;
    (t10900, t10905, t10939, t10948, t10952, t10969, t10971)
}
