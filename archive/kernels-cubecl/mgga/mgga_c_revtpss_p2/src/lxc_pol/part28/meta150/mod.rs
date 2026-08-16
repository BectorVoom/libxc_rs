//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk808;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk809;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk810;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk811;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk812;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta150<F: Float>(t3151: F, t378: F, t335: F, t368: F, t3153: F, t3154: F, t1043: F, t1071: F, t1089: F, t3133: F, t1035: F, t3140: F, t342: F, t357: F, t3259: F, t380: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t3043: F, t3204: F, t3223: F, t3278: F, t3283: F, t3287: F, t3288: F, t3292: F, t3295: F, t3299: F, t381: F, t989: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3300, t3302) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk808::<F>(t3151, t378, t335, t368);
        let (t3303, t3304) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk809::<F>(t3153, t3302, t3154);
        let (t3305, t3309, t3313, t3316) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk810::<F>(t3300, t3304, t1043, t1071, t1089, t3133, t378, t1035, t3140);
        let t3317 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk811::<F>(t3316, t342);
        let t3318 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk812::<F>(t3303, t357);
        let (t3319, t3322, t3325) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk813::<F>(t3300, t3318, t3259, t380, t1024, t1083, t1087, t1090, t1093, t3043, t3204, t3223, t3278, t3283, t3287, t3288, t3292, t3295, t3299, t3305, t3309, t3313, t3317, t342, t381, t989);
    (t3302, t3303, t3304, t3305, t3309, t3313, t3316, t3317, t3318, t3319, t3322, t3325)
}
