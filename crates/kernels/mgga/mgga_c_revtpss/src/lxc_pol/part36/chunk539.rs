//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 539/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk539<F: Float>(t1261: F, t5378: F, t1260: F, t1785: F, t3670: F, t1802: F, t369: F, t475: F, t467: F, t1811: F, t460: F, t1284: F, t1770: F, t354: F, t471: F, t3766: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5379 = t1261 * t5378;
    let t5381 = t1785 * t1260;
    let t5384 = t3670 * t1260;
    let t5389 = t1802 * t369;
    let t5390 = t475 * t5389;
    let t5391 = t467 * t5390;
    let t5417 = t460 * t1811;
    let t5436 = t1770 * t1284;
    let t5457 = t354 * t471;
    let t5462 = t3766 * t487;
    (t5379, t5381, t5384, t5389, t5390, t5391, t5417, t5436, t5457, t5462)
}
