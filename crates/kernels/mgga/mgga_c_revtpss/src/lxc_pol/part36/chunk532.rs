//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 532/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk532<F: Float>(t1469: F, t3362: F, t3367: F, t1130: F, t1719: F, t1723: F, t3390: F, t3407: F, t1729: F, t698: F, t1160: F, t1737: F, t1179: F, t1749: F, t1756: F, t3523: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5046 = t3362 * t1469;
    let t5051 = t3367 * t1469;
    let t5063 = t1719 * t1130;
    let t5071 = t3390 * t1723;
    let t5087 = t3407 * t1723;
    let t5093 = t698 * t1729;
    let t5120 = t1737 * t1160;
    let t5158 = t1749 * t1179;
    let t5184 = t1756 * t3523;
    (t5046, t5051, t5063, t5071, t5087, t5093, t5120, t5158, t5184)
}
