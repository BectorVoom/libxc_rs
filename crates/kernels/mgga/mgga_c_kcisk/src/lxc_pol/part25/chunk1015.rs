//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1015/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1015<F: Float>(t17945: F, t7315: F, t17900: F, t17903: F, t17906: F, t17908: F, t17910: F, t17912: F, t17915: F, t17918: F, t17920: F, t17922: F, t17925: F, t17928: F, t17931: F, t17934: F, t17937: F, t17941: F, t17943: F) -> (F, F) {
    let t17946 = t7315 * t17945;
    let t17948 = t17900 / 8.0 - t17903 / 96.0 - t17906 / 576.0 - t17908 / 72.0 - t17910 / 6.0 + t17912 / 256.0 - t17915 / 48.0 + t17918 / 36.0 + t17920 / 3.0 - t17922 / 16.0 - t17925 / 12.0 + t17928 / 36.0 + t17931 / 576.0 + t17934 / 96.0 - t17937 / 12.0 + t17941 / 864.0 + t17943 / 96.0 + t17946 / 12.0;
    (t17946, t17948)
}
