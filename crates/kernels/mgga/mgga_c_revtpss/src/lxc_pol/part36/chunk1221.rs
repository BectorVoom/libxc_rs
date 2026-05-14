//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1221/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1221<F: Float>(t104379: F, t111457: F, t111532: F, t111592: F, t114246: F, t114264: F, t114270: F, t114301: F, t114305: F, t114311: F, t114313: F, t1469: F, t1923: F, t1927: F, t2123: F, t22671: F, t22688: F, t22699: F, t23842: F, t26776: F, t26792: F, t29355: F, t29554: F, t30682: F, t5819: F, t5825: F, t61: F, t72: F, t7566: F, t7571: F, t7706: F, t7719: F, t8144: F, t8147: F, t92612: F, t96733: F, t96804: F) -> (F,) {
    let t116798 = -t1923 * (-1232.0 / 27.0 * t22699 * t61 - 220.0 / 9.0 * t111592 * t1469 - 20.0 / 9.0 * t104379 * t5819 + 20.0 / 3.0 * t29355 * t5825 + 5.0 / 108.0 * t96733 * t22688 + 5.0 / 6.0 * t26776 * t23842 - 5.0 / 6.0 * t7571 * t22671 + t92612) * t72 * t1927 / 6.0 - t1923 * t30682 * t7719 / 2.0 + 35.0 * t96804 * t114264 + 5.0 / 2.0 * t111532 * t7706 + t114270 * t2123 + 5.0 / 2.0 * t7566 * t114301 + 5.0 / 6.0 * t7566 * t114305 + t111457 * t114311 - 15.0 * t26792 * t114246 + t114313 * t2123 / 3.0 + t29554 * t8144 + t29554 * t8147;
    (t116798,)
}
