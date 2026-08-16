//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 951/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk951<F: Float>(t3054: F, t3073: F, t219: F, t3111: F, t1137: F, t73: F, t8549: F, t9615: F, t8548: F, t9080: F, t9619: F, t3126: F) -> (F, F, F, F, F, F) {
    let t9721 = t3054 * t3073;
    let t9730 = t3111 * t219;
    let t9737 = t1137 * t1137;
    let t9738 = F::cast_from(1.0_f64) / t9737;
    let t9739 = t73 * t9738;
    let t9748 = t8549 * t9615;
    let t9749 = t8548 * t9748;
    let t9751 = t9080 * t9619;
    let t9759 = t3126 * t3073;
    (t9721, t9730, t9739, t9749, t9751, t9759)
}
