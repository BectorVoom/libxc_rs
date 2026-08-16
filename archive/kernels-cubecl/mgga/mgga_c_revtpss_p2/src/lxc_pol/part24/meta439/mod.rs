//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta439<F: Float>(t39768: F, t47065: F, t190: F, t22: F, t519: F, t39762: F, t1317: F, t9545: F, t1340: F, t40129: F, t40182: F, t39821: F) -> (F, F, F, F, F, F, F) {
        let (t47067, t47070, t47072, t47074, t47076, t47084, t47086) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1395::<F>(t39768, t47065, t190, t22, t519, t39762, t1317, t9545, t1340, t40129, t40182, t39821);
    (t47067, t47070, t47072, t47074, t47076, t47084, t47086)
}
