//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2721/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2721<F: Float>(t1261: F, t21110: F, t3172: F, t17401: F, t17620: F, t17728: F, t489: F, t5219: F, t1256: F, t21335: F, t20900: F, t3153: F) -> (F, F, F, F, F) {
    let t70281 = t1261 * t3172 * t21110;
    let t70300 = t17401 * t17620;
    let t70303 = t5219 * t489 * t17728;
    let t70306 = t21335 * t1256;
    let t70311 = t20900 * t3153;
    (t70281, t70300, t70303, t70306, t70311)
}
