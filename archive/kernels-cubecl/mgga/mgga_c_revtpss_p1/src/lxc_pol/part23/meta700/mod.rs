//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta700<F: Float>(t1340: F, t40182: F, t39821: F, t40196: F, t40192: F, t4038: F, t9419: F, t40113: F, t40169: F, t3863: F, t4029: F, t40135: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47084, t47086, t47088, t47092, t47093, t47096, t47098, t47101, t47109) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2450::<F>(t1340, t40182, t39821, t40196, t40192, t4038, t9419, t40113, t40169, t3863, t4029, t40135);
    (t47084, t47086, t47088, t47092, t47093, t47096, t47098, t47101, t47109)
}
