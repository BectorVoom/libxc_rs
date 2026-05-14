//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1078/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1078<F: Float>(t25335: F, t9303: F, t1959: F, t41117: F, t68: F, t785: F, t251: F, t281: F, t1950: F, t2769: F, t786: F, t25404: F, t40270: F, t10115: F, t1951: F, t7058: F, t92871: F) -> (F, F, F, F, F, F, F, F) {
    let t93224 = 0.26019841438354088051e-2 * t9303 * t25335;
    let t93231 = 0.81814717454467823679e-4 * t41117 * t1959;
    let t93238 = t68 * t785;
    let t93240 = t281 * t93238 * t251;
    let t93261 = t786 * t1950 * t2769;
    let t93272 = 0.96373646535613327356e-3 * t40270 * t25404;
    let t93276 = 0.11044544084478153697e-3 * t10115 * t1951;
    let t93278 = 0.22487184191643109717e-1 * t7058 * t92871;
    (t93224, t93231, t93238, t93240, t93261, t93272, t93276, t93278)
}
