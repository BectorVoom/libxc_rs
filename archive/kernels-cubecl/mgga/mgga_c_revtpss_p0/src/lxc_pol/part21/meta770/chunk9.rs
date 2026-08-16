//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2735/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2735<F: Float>(t14662: F, t231: F, t243: F, t2661: F, t2662: F, t14648: F, t14832: F, t2430: F, t10777: F, t10779: F, t14671: F, t14872: F) -> (F, F, F) {
    let t50308 = t2661 * t2662 * t243 * t14662 * t231;
    let t50312 = t2661 * t14832 * t14648 * t2430;
    let t50325 = t10777 * t10779 * t14671 * t14872;
    (t50308, t50312, t50325)
}
