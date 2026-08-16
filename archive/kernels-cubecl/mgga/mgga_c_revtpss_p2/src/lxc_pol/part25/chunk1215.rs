//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1215/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1215<F: Float>(t2311: F, t644: F, t77: F, t2315: F, t640: F, t10410: F, t84: F, t1925: F, t2247: F, t2251: F, t606: F, t2258: F) -> (F, F, F, F, F, F) {
    let t92654 = t77 * t2311 * t644;
    let t92658 = t77 * t640 * t2315;
    let t92662 = t77 * t84 * t10410;
    let t92666 = t2247 * t2251 * t1925;
    let t92669 = t606 * t1925;
    let t92672 = t77 * t84 * t2258;
    (t92654, t92658, t92662, t92666, t92669, t92672)
}
