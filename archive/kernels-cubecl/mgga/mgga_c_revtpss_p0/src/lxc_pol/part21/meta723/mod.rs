//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta723<F: Float>(t1317: F, t9561: F, t1340: F, t40182: F, t39821: F, t40196: F, t40192: F, t4038: F, t9419: F, t40113: F, t40169: F, t2516: F, t9551: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47081, t47084, t47086, t47088, t47092, t47093, t47096, t47098, t47099) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2563::<F>(t1317, t9561, t1340, t40182, t39821, t40196, t40192, t4038, t9419, t40113, t40169, t2516, t9551);
    (t47081, t47084, t47086, t47088, t47092, t47093, t47096, t47098, t47099)
}
