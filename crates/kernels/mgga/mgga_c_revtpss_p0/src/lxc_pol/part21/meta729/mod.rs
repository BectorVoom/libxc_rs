//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta729 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2572;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta729<F: Float>(t221: F, t4019: F, t47293: F, t9995: F, t9905: F, t9976: F, t9984: F, t3978: F, t9921: F, t3926: F, t9909: F, t3930: F, t9901: F, t2661: F, t5675: F, t9929: F, t9934: F, t9775: F, t9981: F, t1398: F, t3992: F, t4010: F, t9956: F, t3938: F, t47218: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47296, t47298, t47302, t47304, t47306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2572::<F>(t221, t4019, t47293, t9995, t9905, t9976, t9984, t3978, t9921, t3926, t9909, t3930, t9901);
        let (t47318, t47320, t47325, t47329) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2573::<F>(t2661, t5675, t9929, t9934, t9775, t9981, t1398, t3992, t4010, t9956, t3938, t47218);
    (t47296, t47298, t47302, t47304, t47306, t47318, t47320, t47325, t47329)
}
