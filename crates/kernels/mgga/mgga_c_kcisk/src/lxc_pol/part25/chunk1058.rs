//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1058/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1058<F: Float>(t17900: F, t17903: F, t17906: F, t17908: F, t17910: F, t17912: F, t17915: F, t17918: F, t17920: F, t17922: F, t17925: F, t17928: F, t17931: F, t17934: F, t17937: F, t17941: F, t17943: F, t17946: F) -> (F,) {
    let t18902 = 0.1875e0 * t17900 - 0.26979166666666666666e-1 * t17903 - 0.44965277777777777777e-2 * t17906 - 0.20833333333333333333e-1 * t17908 - 0.25e0 * t17910 + 0.101171875e-1 * t17912 - 0.53958333333333333333e-1 * t17915 + 0.71944444444444444444e-1 * t17918 + 0.5e0 * t17920 - 0.9375e-1 * t17922 - 0.125e0 * t17925 + 0.41666666666666666666e-1 * t17928 + 0.44965277777777777777e-2 * t17931 + 0.26979166666666666666e-1 * t17934 - 0.125e0 * t17937 + 0.29976851851851851851e-2 * t17941 + 0.26979166666666666666e-1 * t17943 + 0.125e0 * t17946;
    (t18902,)
}
