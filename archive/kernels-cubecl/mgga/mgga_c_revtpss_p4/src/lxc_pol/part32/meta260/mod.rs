//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta260 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1096;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1097;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1098;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1099;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1100;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1101;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta260<F: Float>(t624: F, t72: F, t1927: F, t1923: F, t2047: F, t6977: F, t5: F, t2048: F, t6954: F, t6960: F, t6963: F, t7343: F, t117: F, t116: F, t2051: F, t1310: F, t2055: F, t114: F, t6996: F, t6999: F, t508: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t7348 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1096::<F>(t624, t72);
        let t7349 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1097::<F>(t1927, t7348);
        let (t7351, t7352) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1098::<F>(t1923, t7349, t2047, t6977);
        let (t7356, t7357, t7359) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1099::<F>(t5, t1923, t2048, t6954, t6960, t6963, t7343, t7351, t7352, t117, t116, t2051);
        let t7367 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1100::<F>(t1310, t2055);
        let (t7370, t7373) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1101::<F>(t114, t6996, t6999);
        let t7374 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1102::<F>(t508, t7373);
    (t7348, t7349, t7351, t7352, t7356, t7357, t7359, t7367, t7370, t7373, t7374)
}
