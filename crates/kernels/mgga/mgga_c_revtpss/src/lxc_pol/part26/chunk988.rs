//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 988/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk988<F: Float>(t10406: F, t76: F, t38: F, t45955: F, t2242: F, t2251: F, t2311: F, t644: F, t77: F, t2315: F, t640: F, t10410: F, t84: F, t2258: F, t10327: F, t603: F) -> (F, F, F, F, F, F, F, F) {
    let t92628 = t76 * t10406;
    let t92632 = t45955 * t38;
    let t92639 = t2242 * t2251;
    let t92654 = t77 * t2311 * t644;
    let t92658 = t77 * t640 * t2315;
    let t92662 = t77 * t84 * t10410;
    let t92672 = t77 * t84 * t2258;
    let t92674 = t603 * t10327;
    (t92628, t92632, t92639, t92654, t92658, t92662, t92672, t92674)
}
