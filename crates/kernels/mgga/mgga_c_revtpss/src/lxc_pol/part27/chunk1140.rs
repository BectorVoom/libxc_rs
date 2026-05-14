//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1140/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1140<F: Float>(t1464: F, t7690: F, t2167: F, t4168: F, t27089: F, t575: F, t116: F, t26799: F, t2327: F, t7583: F, t10368: F, t55: F, t10326: F, t10356: F, t11231: F, t1923: F, t1927: F, t2122: F, t2123: F, t25117: F, t25150: F, t26776: F, t26782: F, t26783: F, t26786: F, t26789: F, t6954: F, t6977: F, t72: F, t7571: F, t7576: F, t7579: F, t92612: F, t92628: F, t92632: F) -> (F, F, F, F, F, F) {
    let t96690 = t7690 * t1464;
    let t96692 = t2167 * t4168;
    let t96694 = t27089 * t575;
    let t96706 = t26799 * t116;
    let t96709 = t7583 * t2327;
    let t96733 = t55 * t10368;
    let t96748 = -t1923 * t2122 * t92628 / 6.0 + t25117 * t7576 + t25117 * t7579 - t92632 * t2123 / 6.0 - t25150 * t7576 / 2.0 - t25150 * t7579 / 2.0 - t6954 * t26783 / 2.0 - t6954 * t26786 - t6954 * t26789 / 2.0 - t1923 * (5.0 / 108.0 * t96733 * t10356 + 5.0 / 6.0 * t26776 * t11231 - 5.0 / 6.0 * t7571 * t10326 + t92612) * t72 * t1927 / 6.0 - t1923 * t26782 * t6977 / 2.0;
    (t96690, t96692, t96694, t96706, t96709, t96748)
}
