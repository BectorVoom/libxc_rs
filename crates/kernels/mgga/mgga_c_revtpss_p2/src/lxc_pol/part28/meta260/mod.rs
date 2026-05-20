//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1158;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1159;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1160;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1161;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1162;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1163;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1164;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta260<F: Float>(t1032: F, t251: F, t867: F, t786: F, t1958: F, t72: F, t686: F, t1954: F, t2452: F, t1955: F, t860: F, t233: F, t2769: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t7056 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1158::<F>(t1032, t251);
        let t7057 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1159::<F>(t7056, t867);
        let t7058 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1160::<F>(t7057, t786);
        let (t7059, t7060) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1161::<F>(t1958, t72, t686);
        let (t7062, t7063) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1162::<F>(t7058, t7060, t1954, t2452);
        let t7064 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1163::<F>(t7057, t7063);
        let (t7066, t7067, t7070) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1164::<F>(t7060, t7064, t1955, t860, t7056);
        let t7071 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1165::<F>(t233, t2769);
    (t7056, t7057, t7058, t7059, t7060, t7062, t7063, t7064, t7066, t7067, t7070, t7071)
}
