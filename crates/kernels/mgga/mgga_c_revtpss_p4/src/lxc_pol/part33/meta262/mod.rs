//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta262 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1170;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1171;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1172;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1173;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1174;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1175;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta262<F: Float>(t1032: F, t555: F, t1426: F, t786: F, t2029: F, t72: F, t686: F, t7063: F, t1419: F, t1955: F, t4075: F, t545: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t7282 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1170::<F>(t1032, t555);
        let t7283 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1171::<F>(t1426, t7282);
        let t7284 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1172::<F>(t7283, t786);
        let (t7285, t7286) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1173::<F>(t2029, t72, t686);
        let (t7288, t7289) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1174::<F>(t7284, t7286, t7063, t7283);
        let (t7291, t7292, t7295) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1175::<F>(t7286, t7289, t1419, t1955, t7282);
        let t7296 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1176::<F>(t4075, t545);
    (t7282, t7283, t7284, t7285, t7286, t7288, t7289, t7291, t7292, t7295, t7296)
}
