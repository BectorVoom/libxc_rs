//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk779;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta147<F: Float>(t1020: F, t1053: F, t1021: F, t1058: F, t225: F, t3043: F, t366: F, t371: F, t373: F, t676: F, t367: F, t3057: F, t3059: F, t372: F, t1024: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3191, t3194, t3196, t3197, t3201, t3203, t3204) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk779::<F>(t1020, t1053, t1021, t1058, t225, t3043, t366, t371, t373, t676, t367, t3057);
        let (t3205, t3206, t3208, t3211) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk780::<F>(t3204, t366, t3059, t373, t371, t372, t1024, t1053);
    (t3191, t3194, t3196, t3197, t3201, t3203, t3204, t3205, t3206, t3208, t3211)
}
