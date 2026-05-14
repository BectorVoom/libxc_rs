//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1114/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1114<F: Float>(t3275: F, t3472: F, t39178: F, t11325: F, t11518: F, t3262: F, t11189: F, t40289: F, t3465: F, t40667: F, t12060: F, t37271: F, t12206: F, t37282: F, t38251: F, t38259: F, t38261: F, t39109: F, t39113: F, t39114: F, t39115: F, t40642: F, t42277: F) -> (F, F, F, F, F, F, F) {
    let t42281 = 5.0 / 16.0 * t3275 * t3472 * t39178;
    let t42284 = 15.0 / 8.0 * t3262 * t11325 * t11518;
    let t42287 = 45.0 / 64.0 * t3275 * t11189 * t40289;
    let t42290 = 3.0 / 2.0 * t3275 * t3465 * t40667;
    let t42292 = 5.0 / 8.0 * t37271 * t12060;
    let t42294 = 3.0 / 2.0 * t37282 * t12206;
    let t42298 = t42277 - t39109 - 0.32326021979378162576e-5 * t38251 - t42281 + t42284 - t42287 + t42290 - t42292 + t42294 - 0.60975299583150056624e-3 * t38259 + 0.60975299583150056624e-3 * t38261 - t39113 - t39114 - t39115 + 0.60975299583150056624e-3 * t40642;
    (t42281, t42284, t42287, t42290, t42292, t42294, t42298)
}
