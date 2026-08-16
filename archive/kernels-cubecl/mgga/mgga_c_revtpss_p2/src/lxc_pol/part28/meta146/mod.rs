//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk791;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk792;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk793;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta146<F: Float>(t1020: F, t1053: F, t1021: F, t1058: F, t225: F, t3043: F, t366: F, t371: F, t373: F, t676: F, t367: F, t3057: F, t3059: F, t372: F, t1024: F, t1026: F, t127: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3191, t3194, t3196, t3197, t3201) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk791::<F>(t1020, t1053, t1021, t1058, t225, t3043, t366, t371, t373, t676);
        let (t3203, t3204) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk792::<F>(t3201, t367, t225, t3057);
        let (t3205, t3206, t3208, t3211) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk793::<F>(t3204, t366, t3059, t373, t371, t372, t1024, t1053);
        let t3215 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk794::<F>(t1026, t127, t371);
    (t3191, t3194, t3196, t3197, t3201, t3203, t3204, t3205, t3206, t3208, t3211, t3215)
}
