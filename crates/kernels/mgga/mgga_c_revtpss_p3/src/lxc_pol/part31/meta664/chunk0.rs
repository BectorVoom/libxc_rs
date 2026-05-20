//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2254/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2254<F: Float>(t28264: F, t572: F, t5920: F, t105886: F, t117: F, t2042: F, t22544: F, t26123: F, t5883: F, t7002: F, t101622: F, t1518: F) -> (F, F, F, F, F, F) {
    let t109310 = F::new(6.0) * t572 * t28264 * t5920;
    let t109315 = F::new(3.0) * t572 * t117 * t105886;
    let t109319 = F::new(3.0) * t22544 * t2042;
    let t109322 = F::new(6.0) * t572 * t26123 * t5920;
    let t109327 = F::new(6.0) * t572 * t5883 * t7002;
    let t109330 = F::new(12.0) * t572 * t101622 * t1518;
    (t109310, t109315, t109319, t109322, t109327, t109330)
}
