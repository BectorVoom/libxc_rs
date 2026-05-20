//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta672<F: Float>(t16505: F, t994: F, t11627: F, t42859: F, t342: F, t11620: F, t4982: F, t12077: F, t989: F, t12153: F, t3057: F, t3043: F, t3316: F) -> (F, F, F, F, F, F, F) {
        let (t43528, t43536, t43537, t43562, t43574, t43598, t43611) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2473::<F>(t16505, t994, t11627, t42859, t342, t11620, t4982, t12077, t989, t12153, t3057, t3043, t3316);
    (t43528, t43536, t43537, t43562, t43574, t43598, t43611)
}
