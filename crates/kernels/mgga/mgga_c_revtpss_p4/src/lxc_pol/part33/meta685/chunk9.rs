//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2272/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2272<F: Float>(t109291: F, t109293: F, t109295: F, t109299: F, t109305: F, t109307: F, t109310: F, t109315: F, t109319: F, t109322: F, t109327: F, t109330: F, t109333: F, t113015: F, t1461: F, t1918: F, t29480: F, t30985: F, t573: F, t5805: F, t6948: F, t7696: F, t8245: F, param_d: F) -> F {
    let t113050 = t113015 * t573 * param_d + F::new(3.0) * t1461 * t30985 + F::new(6.0) * t1918 * t29480 + F::new(6.0) * t5805 * t8245 + F::new(3.0) * t6948 * t7696 + t109291 + t109293 + t109295 + t109299 + t109305 + t109307 + t109310 + t109315 + t109319 + t109322 + t109327 + t109330 + t109333;
    t113050
}
