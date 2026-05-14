//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 540/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk540<F: Float>(t681: F, t1394: F, t429: F, t686: F, t3841: F, t435: F, t690: F, t3845: F, t698: F, t445: F, t5082: F, t1060: F, t696: F, t213: F, t695: F, t1849: F, t967: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5100 = t681 * t681;
    let t5101 = 1.0 / t5100;
    let t5122 = 0.8197e-2 * t429 * t1394 * t686;
    let t5125 = 0.21133333333333333333e-2 * t435 * t3841 * t690;
    let t5126 = t3845 * t698;
    let t5128 = 0.16804375e-4 * t445 * t5126;
    let t5129 = 0.23911438650126355246e-1 * t5082;
    let t5130 = t696 * t1060;
    let t5134 = t213 * t695;
    let t5135 = 0.15538616723388920628e-3 * t5134;
    let t5136 = t967 * t1849;
    (t5100, t5101, t5122, t5125, t5128, t5129, t5130, t5134, t5135, t5136)
}
