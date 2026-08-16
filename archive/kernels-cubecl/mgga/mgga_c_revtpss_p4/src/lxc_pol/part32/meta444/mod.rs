//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta444<F: Float>(t3930: F, t6846: F, t221: F, t4019: F, t6862: F, t10001: F, t6800: F, t72: F, t757: F, t1317: F, t6801: F, t13599: F, t21901: F, t21905: F, t21933: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F) {
        let (t22179, t22182, t22183, t22187, t22189, t22190) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1614::<F>(t3930, t6846, t221, t4019, t6862, t10001, t6800, t72, t757, t1317, t6801, t13599, t21901, t21905, t21933, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22179, t22182, t22183, t22187, t22189, t22190)
}
