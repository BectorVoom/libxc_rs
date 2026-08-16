//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1608;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta442<F: Float>(t221: F, t6836: F, t9921: F, t3978: F, t125: F, t6816: F, t1399: F, t3936: F, t6843: F, t3938: F, t5673: F, t21990: F, t5674: F, t13944: F, t6869: F, t543: F, t5591: F, t13848: F, t9818: F, t9816: F, t13798: F, t13801: F, t13810: F, t13813: F, t3934: F, t5671: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22068, t22069, t22076, t22079, t22081, t22085, t22089) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1608::<F>(t221, t6836, t9921, t3978, t125, t6816, t1399, t3936, t6843, t3938, t5673, t21990, t5674);
        let (t22093, t22098, t22102, t22105) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1609::<F>(t13944, t3936, t6869, t543, t5591, t5674, t13848, t9818, t9816, t13798, t13801, t13810, t13813, t22069, t22076, t22081, t22085, t22089, t3934, t5671);
    (t22068, t22076, t22079, t22081, t22085, t22089, t22093, t22098, t22102, t22105)
}
