//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1253/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1253<F: Float>(t25411: F, t93225: F, t1959: F, t41117: F, t68: F, t785: F, t251: F, t281: F, t25410: F, t7078: F, t10910: F, t1955: F) -> (F, F, F, F, F) {
    let t93228 = t25411 * t93225;
    let t93231 = F::new(0.81814717454467823679e-4) * t41117 * t1959;
    let t93238 = t68 * t785;
    let t93240 = t281 * t93238 * t251;
    let t93242 = t93240 * t25410 * t7078;
    let t93244 = t1955 * t10910;
    (t93228, t93231, t93238, t93242, t93244)
}
