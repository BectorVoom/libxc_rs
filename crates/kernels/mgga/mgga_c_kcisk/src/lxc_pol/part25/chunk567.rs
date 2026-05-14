//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 567/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk567<F: Float>(t4658: F, t5101: F, t1814: F, t4684: F, t1835: F, t4644: F, t1842: F, t1856: F, t1394: F, t429: F, t686: F, t3841: F, t435: F, t690: F, t3845: F, t698: F) -> (F, F, F, F, F, F, F, F) {
    let t5102 = t5101 * t4658;
    let t5105 = t1814 * t4684;
    let t5111 = t1835 * t4644;
    let t5114 = t1842 * t4644;
    let t5117 = t1856 * t4644;
    let t5122 = 0.8197e-2 * t429 * t1394 * t686;
    let t5125 = 0.21133333333333333333e-2 * t435 * t3841 * t690;
    let t5126 = t3845 * t698;
    (t5102, t5105, t5111, t5114, t5117, t5122, t5125, t5126)
}
