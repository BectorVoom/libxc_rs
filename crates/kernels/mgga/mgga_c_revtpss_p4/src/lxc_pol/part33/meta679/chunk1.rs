//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2213/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2213<F: Float>(t1916: F, t28277: F, t1518: F, t572: F, t670: F, t7741: F, t28280: F, t1459: F, t30191: F, t28264: F, t5920: F, t105886: F, t117: F) -> (F, F, F, F, F, F) {
    let t109295 = F::new(12.0) * t1916 * t28277;
    let t109299 = F::new(12.0) * t572 * t670 * t7741 * t1518;
    let t109305 = F::new(6.0) * t1916 * t28280;
    let t109307 = F::new(6.0) * t1459 * t30191;
    let t109310 = F::new(6.0) * t572 * t28264 * t5920;
    let t109315 = F::new(3.0) * t572 * t117 * t105886;
    (t109295, t109299, t109305, t109307, t109310, t109315)
}
