//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta67 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk401;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk402;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk403;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk404;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk405;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk406;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta67<F: Float>(t493: F, t225: F, t473: F, t487: F, t1214: F, t1032: F, t1243: F, t460: F, t355: F, t471: F, t1248: F, t1269: F, t489: F, t1204: F, t1234: F, t490: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1275, t1276, t1277) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk401::<F>(t493, t225);
        let t1280 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk402::<F>(t473, t487);
        let (t1281, t1284) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk403::<F>(t1214, t1280, t1032, t1243);
        let t1285 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk404::<F>(t1284, t460);
        let t1287 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk405::<F>(t355, t471);
        let (t1288, t1291, t1294) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk406::<F>(t1248, t1287, t487, t1269, t489, t1204, t1234, t1281, t1285, t460, t490);
        let t1295 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk407::<F>(t1277, t1294);
    (t1275, t1276, t1277, t1280, t1281, t1284, t1285, t1287, t1288, t1291, t1294, t1295)
}
