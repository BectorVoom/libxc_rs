//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta53 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk356;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk357;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk358;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta53<F: Float>(t1320: F, t521: F, t520: F, t749: F, t512: F, t72: F, t757: F, t177: F, t762: F, t531: F, t566: F, t513: F, t516: F, t212: F, t555: F, t225: F, t561: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1322, t1333) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk356::<F>(t1320, t521, t520, t749);
        let (t1334, t1337, t1339, t1340) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk357::<F>(t1333, t512, t520, t72, t757, t177);
        let (t1342, t1343, t1344, t1348, t1357) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk358::<F>(t1340, t762, t531, t566, t513, t516, t212, t555);
        let t1358 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk359::<F>(t225, t561);
    (t1322, t1333, t1334, t1337, t1339, t1340, t1342, t1343, t1344, t1348, t1357, t1358)
}
