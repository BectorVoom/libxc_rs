//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta69 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk416;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk417;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk418;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk419;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk420;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta69<F: Float>(t1331: F, t512: F, t520: F, t749: F, t1330: F, t187: F, t72: F, t757: F, t177: F, t762: F, t531: F, t566: F, t513: F, t30: F, t605: F, t516: F, zeta_threshold: F, t33: F, t1113: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1332, t1333) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk416::<F>(t1331, t512, t520, t749);
        let (t1334, t1336, t1337, t1339, t1340) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk417::<F>(t1333, t512, t1330, t187, t520, t72, t757, t177);
        let (t1342, t1343) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk418::<F>(t1340, t762, t531, t566);
        let t1344 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk419::<F>(t513);
        let (t1347, t1348) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk420::<F>(t30, t1344, t605, t516, zeta_threshold);
        let t1353 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk421::<F>(t33, t1113, t1348, t1347, zeta_threshold);
    (t1332, t1333, t1334, t1336, t1337, t1339, t1340, t1342, t1343, t1344, t1348, t1353)
}
