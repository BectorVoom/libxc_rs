//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta70 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk488;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk489;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk490;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk491;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk492;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk493;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk494;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta70<F: Float>(t530: F, t549: F, t240: F, t72: F, t1353: F, t828: F, t1368: F, t1370: F, t1372: F, t1378: F, t1383: F, t1388: F, t1401: F, t1407: F, t1410: F, t225: F, t561: F, t213: F, t555: F, t560: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1412 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk488::<F>(t530, t549);
        let t1413 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk489::<F>(t1412, t240);
        let t1414 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk490::<F>(t1413, t72);
        let (t1416, t1419) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk491::<F>(t1353, t1414, t828, t1368, t1370, t1372, t1378, t1383, t1388, t1401, t1407, t1410);
        let t1420 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk492::<F>(t1419, t225);
        let (t1421, t1424) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk493::<F>(t1420, t561, t213, t555);
        let (t1425, t1426) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk494::<F>(t560);
        let t1427 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk495::<F>(t1426, t225);
    (t1412, t1413, t1414, t1416, t1419, t1420, t1421, t1424, t1425, t1426, t1427)
}
