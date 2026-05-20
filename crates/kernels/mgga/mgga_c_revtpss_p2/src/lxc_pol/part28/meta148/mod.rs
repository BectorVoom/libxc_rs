//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk798;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk799;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk800;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk801;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk802;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta148<F: Float>(t1016: F, t140: F, t1011: F, t1015: F, t2258: F, t1012: F, t271: F, t905: F, t2852: F, t2251: F, t1017: F, t1025: F, t1028: F, t1068: F, t3188: F, t3191: F, t3194: F, t3197: F, t3203: F, t3205: F, t3208: F, t3211: F, t3216: F, t3220: F, t3224: F, t3231: F, t3234: F, t3238: F, t3241: F, t375: F, t3187: F, t225: F, t385: F, t1071: F, t342: F, t1077: F, t384: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3244, t3245, t3247, t3248, t3252) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk798::<F>(t1016, t140, t1011, t1015, t2258, t1012, t271, t905);
        let (t3254, t3255, t3258) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk799::<F>(t2852, t3252, t2251, t1012, t1011, t1017, t1025, t1028, t1068, t3188, t3191, t3194, t3197, t3203, t3205, t3208, t3211, t3216, t3220, t3224, t3231, t3234, t3238, t3241, t3245, t3248, t375);
        let t3259 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk800::<F>(t3187, t3258);
        let (t3261, t3264, t3268) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk801::<F>(t225, t3259, t385, t1071, t342, t1077, t384);
        let t3269 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk802::<F>(t225, t3268);
    (t3244, t3245, t3247, t3248, t3252, t3254, t3255, t3259, t3261, t3264, t3268, t3269)
}
