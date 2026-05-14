//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 499/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk499<F: Float>(t480: F, t5326: F, t3623: F, t4890: F, t3782: F, t1794: F, t3153: F, t3767: F, t73: F, t140: F, t1781: F, t1222: F, t127: F, t1789: F, t371: F, t1235: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5327 = t5326 * t480;
    let t5330 = t3623 * t4890;
    let t5331 = t3782 * t5330;
    let t5332 = t1794 * t3153;
    let t5340 = t3767 * t5330;
    let t5351 = t1794 * t73;
    let t5357 = t140 * t1781;
    let t5358 = t1222 * t5357;
    let t5362 = t371 * t127 * t1789;
    let t5363 = t1235 * t5362;
    (t5327, t5330, t5331, t5332, t5340, t5351, t5357, t5358, t5362, t5363)
}
