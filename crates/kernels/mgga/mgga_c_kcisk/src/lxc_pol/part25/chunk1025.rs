//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1025/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1025<F: Float>(t18041: F, t18087: F, t18123: F, t18160: F, t716: F, t736: F, t17156: F, t5322: F, t5321: F, t16711: F, t7303: F, t5289: F, t17949: F, t17952: F, t17954: F, t17956: F, t17958: F, t17962: F, t17964: F, t17967: F, t17970: F, t17973: F, t17977: F, t17980: F, t17984: F, t17987: F, t17989: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t18162 = t18041 + t18087 + t18123 + t18160;
    let t18163 = t18162 * t716;
    let t18164 = t18163 * sigma2;
    let t18165 = t18164 * t736;
    let t18167 = t5322 * t17156;
    let t18168 = t5321 * t18167;
    let t18170 = t7303 * t16711;
    let t18171 = t5289 * t18170;
    let t18173 = t17949 / 96.0 + t17952 / 4.0 - t17954 / 12.0 + t17956 / 256.0 + t17958 / 48.0 - t17962 / 256.0 + t17964 / 18.0 - t17967 / 24.0 + 2.0 / 9.0 * t17970 + t17973 / 192.0 - t17977 / 18.0 + t17980 / 54.0 + t17984 / 8.0 - t17987 / 288.0 - t17989 / 12.0 + t18165 / 16.0 + t18168 / 192.0 + t18171 / 72.0;
    (t18163, t18165, t18167, t18168, t18170, t18171, t18173)
}
