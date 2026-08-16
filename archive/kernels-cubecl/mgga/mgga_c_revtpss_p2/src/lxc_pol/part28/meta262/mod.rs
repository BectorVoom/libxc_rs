//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta262 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1171;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1172;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1173;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1174;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta262<F: Float>(t343: F, t613: F, t136: F, t1007: F, t1968: F, t1967: F, t800: F, t1020: F, t1972: F, t1024: F, t1035: F, t1039: F, sigma0: F, t1033: F, t1052: F, t1971: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7105, t7106, t7110, t7111) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1171::<F>(t343, t613, t136, t1007, t1968, t1967, t800);
        let (t7114, t7117) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1172::<F>(t1020, t1972, t1024);
        let (t7120, t7121) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1173::<F>(t1035, t1039, sigma0);
        let t7122 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1174::<F>(t1033, t7121);
        let t7125 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1175::<F>(t1052, t1971);
    (t7105, t7106, t7110, t7111, t7114, t7117, t7120, t7121, t7122, t7125)
}
