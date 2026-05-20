//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1424;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1425;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1426;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta375<F: Float>(t33: F, t1711: F, t9350: F, t2: F, t3841: F, t1113: F, t580: F, t22: F, t3351: F, t3842: F, t516: F, t5557: F, t5560: F, zeta_threshold: F, t13564: F, t162: F, t187: F, t1857: F, t3857: F, t5591: F, t566: F, t9375: F, t177: F, t5566: F, t762: F, t1450: F, t5778: F, t3889: F, t5537: F, t1353: F, t1868: F, t3829: F, t4139: F, t5532: F, t5536: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t9547: F, t9599: F) -> (F, F, F, F, F, F, F) {
        let (t13569, t13579) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1424::<F>(t33, t1711, t9350, t2, t3841, t1113, t580, t22, t3351, t3842, t516, t5557, t5560, zeta_threshold);
        let (t13581, t13583, t13585, t13586, t13593, t13599) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1425::<F>(t13564, t13579, t162, t187, t1857, t3857, t5591, t566, t9375, t177, t5566, t762);
        let t13610 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1426::<F>(t1450, t5778, t3889, t5537, t1353, t13583, t13585, t13586, t13593, t13599, t1868, t3829, t4139, t5532, t5536, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391, t9547, t9599);
    (t13569, t13581, t13583, t13585, t13593, t13599, t13610)
}
