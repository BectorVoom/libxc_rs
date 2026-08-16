//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta149 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk803;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk804;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk805;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk806;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta149<F: Float>(t1096: F, t3269: F, t1086: F, t989: F, t1082: F, t3059: F, t378: F, t994: F, t1089: F, t3118: F, t1071: F, t359: F, t999: F, t3075: F, t3140: F, t3143: F, t342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3270 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk803::<F>(t1096);
        let t3271 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk804::<F>(t3269, t3270);
        let t3278 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk805::<F>(t1086, t989);
        let (t3283, t3286, t3287, t3288, t3291, t3292, t3295, t3298) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk806::<F>(t1082, t3059, t1086, t378, t994, t1089, t3118, t1071, t359, t999, t3075, t3140, t3143);
        let t3299 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk807::<F>(t3298, t342);
    (t3270, t3271, t3278, t3283, t3286, t3287, t3288, t3291, t3292, t3295, t3298, t3299)
}
