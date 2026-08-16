//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1997/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1997<F: Float>(t2453: F, t25309: F, t25301: F, t25304: F, t251: F, t25410: F, t136: F, t137: F, t1949: F, t2438: F, t837: F, t25305: F, t92894: F) -> (F, F, F, F, F, F, F, F) {
    let t93157 = t2453 * t25309;
    let t93158 = t93157 * t25301;
    let t93160 = t25304 * t25309;
    let t93161 = t93160 * t25301;
    let t93169 = t2453 * t251;
    let t93170 = t93169 * t25410;
    let t93172 = t1949 * t136 * t137;
    let t93173 = t2438 * t837;
    let t93174 = t93172 * t93173;
    let t93175 = t93170 * t93174;
    let t93177 = t25305 * t92894;
    (t93158, t93161, t93169, t93170, t93172, t93174, t93175, t93177)
}
