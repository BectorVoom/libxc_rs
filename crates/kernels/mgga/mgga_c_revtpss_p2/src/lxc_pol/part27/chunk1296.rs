//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1296/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1296<F: Float>(t116: F, t26799: F, t2327: F, t7583: F, t10368: F, t55: F, t10326: F, t10356: F, t11231: F, t1923: F, t1927: F, t2122: F, t2123: F, t25117: F, t25150: F, t26776: F, t26782: F, t26783: F, t26786: F, t26789: F, t6954: F, t6977: F, t72: F, t7571: F, t7576: F, t7579: F, t92612: F, t92628: F, t92632: F) -> (F, F, F) {
    let t96706 = t26799 * t116;
    let t96709 = t7583 * t2327;
    let t96733 = t55 * t10368;
    let t96748 = -t1923 * t2122 * t92628 / F::new(6.0) + t25117 * t7576 + t25117 * t7579 - t92632 * t2123 / F::new(6.0) - t25150 * t7576 / F::new(2.0) - t25150 * t7579 / F::new(2.0) - t6954 * t26783 / F::new(2.0) - t6954 * t26786 - t6954 * t26789 / F::new(2.0) - t1923 * (F::new(5.0) / F::new(108.0) * t96733 * t10356 + F::new(5.0) / F::new(6.0) * t26776 * t11231 - F::new(5.0) / F::new(6.0) * t7571 * t10326 + t92612) * t72 * t1927 / F::new(6.0) - t1923 * t26782 * t6977 / F::new(2.0);
    (t96706, t96709, t96748)
}
