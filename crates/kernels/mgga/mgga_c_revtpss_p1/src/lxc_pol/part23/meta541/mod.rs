//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2089;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta541<F: Float>(t22096: F, t3936: F, t5674: F, t13848: F, t6869: F, t9818: F, t9816: F, t13798: F, t13801: F, t13810: F, t13813: F, t22069: F, t22076: F, t22081: F, t22085: F, t22089: F, t22093: F, t3934: F, t5671: F, t22046: F, t3938: F, t5659: F, t5673: F, t1399: F, t125: F, t6836: F, t9955: F, t1413: F, t6816: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22098, t22102, t22103, t22105) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2089::<F>(t22096, t3936, t5674, t13848, t6869, t9818, t9816, t13798, t13801, t13810, t13813, t22069, t22076, t22081, t22085, t22089, t22093, t3934, t5671);
        let (t22107, t22111, t22115, t22120, t22125) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2090::<F>(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
    (t22098, t22102, t22103, t22105, t22107, t22111, t22115, t22120, t22125)
}
