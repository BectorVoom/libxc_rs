//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta724 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta724<F: Float>(t3863: F, t4029: F, t177: F, t762: F, t9363: F, t1340: F, t40135: F, t4038: F, t9425: F, t1330: F, t512: F, t9544: F) -> (F, F, F, F, F) {
        let (t47101, t47106, t47109, t47110, t47113) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2564::<F>(t3863, t4029, t177, t762, t9363, t1340, t40135, t4038, t9425, t1330, t512, t9544);
    (t47101, t47106, t47109, t47110, t47113)
}
