//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta806 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta806<F: Float>(t3863: F, t4029: F, t1340: F, t40135: F, t4038: F, t9425: F, t1330: F, t512: F, t9544: F, t3869: F, t39739: F, t39430: F) -> (F, F, F, F, F, F) {
        let (t47101, t47109, t47110, t47113, t47116, t47118) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2908::<F>(t3863, t4029, t1340, t40135, t4038, t9425, t1330, t512, t9544, t3869, t39739, t39430);
    (t47101, t47109, t47110, t47113, t47116, t47118)
}
