//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta68 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk442;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk443;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk444;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk445;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta68<F: Float>(t1317: F, t521: F, t19: F, t588: F, t30: F, t33: F, t513: F, t605: F, t1113: F, t516: F, t162: F, zeta_threshold: F, t189: F, t512: F, t520: F, t749: F, t187: F, t72: F, t757: F, t177: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1319, t1320) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk442::<F>(t1317, t521, t19, t588);
        let (t1322, t1330) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk443::<F>(t30, t33, t1320, t521, t513, t605, t1113, t516, t162, zeta_threshold);
        let t1331 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk444::<F>(t1330, t189);
        let (t1332, t1333) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk445::<F>(t1331, t512, t520, t749);
        let (t1334, t1336, t1337, t1339, t1340) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk446::<F>(t1333, t512, t1330, t187, t520, t72, t757, t177);
    (t1319, t1320, t1322, t1330, t1331, t1332, t1333, t1334, t1336, t1337, t1339, t1340)
}
