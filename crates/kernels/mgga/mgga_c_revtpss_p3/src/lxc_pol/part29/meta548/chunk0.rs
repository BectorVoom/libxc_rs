//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1885/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1885<F: Float>(t25904: F, t96248: F, t26230: F, t9681: F, t94674: F, t2470: F, t26270: F, t7284: F, t96220: F, t9675: F, t94771: F, t7514: F, t9288: F) -> (F, F, F, F, F, F, F, F) {
    let t96269 = t25904 * t96248;
    let t96271 = t26230 * t9681;
    let t96272 = t94674 * t96271;
    let t96276 = t26270 * t2470;
    let t96277 = t7284 * t96276;
    let t96279 = t96220 * t9675;
    let t96280 = t94771 * t96279;
    let t96282 = t7514 * t9288;
    (t96269, t96271, t96272, t96276, t96277, t96279, t96280, t96282)
}
