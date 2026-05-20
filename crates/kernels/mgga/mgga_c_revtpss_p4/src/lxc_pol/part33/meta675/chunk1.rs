//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2207/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2207<F: Float>(t28187: F, t7898: F, t30110: F, t531: F, t2014: F, t7238: F, t28043: F, t7732: F, t28021: F, t28173: F, t1937: F, t75439: F) -> (F, F, F, F, F, F) {
    let t109172 = F::new(2.0) * t7898 * t28187;
    let t109173 = t531 * t30110;
    let t109176 = F::new(3.0) * t2014 * t109173 * t7238;
    let t109178 = F::new(4.0) * t7732 * t28043;
    let t109180 = F::new(2.0) * t7898 * t28021;
    let t109182 = F::new(6.0) * t7898 * t28173;
    let t109194 = F::new(2.0) * t75439 * t1937;
    (t109172, t109176, t109178, t109180, t109182, t109194)
}
