//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1749;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta459<F: Float>(t1317: F, t9561: F, t1340: F, t40182: F, t39821: F, t40196: F, t9554: F, t40192: F, t4038: F, t9419: F, t40113: F, t40169: F, t2516: F, t9551: F, t3863: F, t4029: F, t39989: F, t40067: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1749::<F>(t1317, t9561, t1340, t40182, t39821, t40196, t9554, t40192, t4038, t9419, t40113, t40169);
        let (t47100, t47102, t47103) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1750::<F>(t2516, t9551, t3863, t4029, t39989, t40067, t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098);
    (t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098, t47100, t47102, t47103)
}
