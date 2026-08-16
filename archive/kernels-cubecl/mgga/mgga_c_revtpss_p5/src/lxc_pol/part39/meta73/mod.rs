//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk441;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk442;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk443;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta73<F: Float>(t1353: F, t1414: F, t828: F, t1368: F, t1370: F, t1372: F, t1378: F, t1383: F, t1388: F, t1401: F, t1407: F, t1410: F, t225: F, t561: F, t213: F, t555: F, t560: F, t545: F, t869: F, t689: F, t546: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1416, t1419) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk441::<F>(t1353, t1414, t828, t1368, t1370, t1372, t1378, t1383, t1388, t1401, t1407, t1410);
        let (t1420, t1421, t1424) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk442::<F>(t1419, t225, t561, t213, t555);
        let (t1425, t1426, t1427) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk443::<F>(t560, t225);
        let (t1428, t1429, t1431, t1432) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk444::<F>(t545, t555, t869, t689, t546, t786);
    (t1416, t1419, t1420, t1421, t1424, t1425, t1426, t1427, t1428, t1429, t1431, t1432)
}
