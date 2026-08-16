//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1068/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1068<F: Float>(t1209: F, t1811: F, t1256: F, t1804: F, t1786: F, t1230: F, t1803: F, t225: F, t5216: F, t480: F, t1796: F, t3172: F) -> (F, F, F, F, F, F, F) {
    let t5251 = t1209 * t1811;
    let t5254 = t1804 * t1256;
    let t5256 = t1786 * t1256;
    let t5258 = t1230 * t1803;
    let t5261 = t5216 * t225;
    let t5262 = t5261 * t480;
    let t5265 = t3172 * t1796;
    (t5251, t5254, t5256, t5258, t5261, t5262, t5265)
}
