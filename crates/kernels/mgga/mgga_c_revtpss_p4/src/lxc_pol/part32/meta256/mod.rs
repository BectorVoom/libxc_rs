//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1074;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1075;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1076;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1077;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1078;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1079;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta256<F: Float>(t7056: F, t867: F, t786: F, t1954: F, t2452: F, t1955: F, t860: F, t233: F, t2769: F, t822: F) -> (F, F, F, F, F, F, F, F) {
        let t7057 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1074::<F>(t7056, t867);
        let t7058 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1075::<F>(t7057, t786);
        let t7063 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1076::<F>(t1954, t2452);
        let t7064 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1077::<F>(t7057, t7063);
        let (t7067, t7070) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1078::<F>(t1955, t860, t7056);
        let t7071 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1079::<F>(t233, t2769);
        let t7076 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1080::<F>(t822, t867);
    (t7057, t7058, t7063, t7064, t7067, t7070, t7071, t7076)
}
