//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta423 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1496;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1497;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1498;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1499;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1500;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1501;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta423<F: Float>(t6426: F, t689: F, t6430: F, t1120: F, t20272: F, t128: F, t12256: F, t5819: F, t606: F, t12305: F, t12268: F, t3360: F, t4186: F, t5046: F, t6421: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t20285 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1496::<F>(t6426, t689);
        let t20287 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1497::<F>(t6430, t689);
        let t20290 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1498::<F>(t1120, t20272, t128);
        let (t20293, t20295) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1499::<F>(t12256, t5819, t606, t12305, t128);
        let (t20298, t20300) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1500::<F>(t12268, t5819, t606, t3360, t128);
        let (t20302, t20304) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1501::<F>(t4186, t5046, t3360, t128);
        let (t20306, t20308) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1502::<F>(t606, t6421, t1120, t128);
    (t20285, t20287, t20290, t20293, t20295, t20298, t20300, t20302, t20304, t20306, t20308)
}
