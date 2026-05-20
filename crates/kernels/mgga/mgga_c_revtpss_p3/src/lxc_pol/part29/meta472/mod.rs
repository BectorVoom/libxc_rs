//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta472<F: Float>(t25394: F, t26550: F, t2061: F, t25402: F, t7056: F, t10073: F, t26544: F, t7064: F, t7384: F, t887: F, t689: F, t7399: F, t786: F) -> (F, F, F, F, F, F, F, F) {
        let (t26551, t26554, t26555, t26557, t26558, t26560, t26561, t26563) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1737::<F>(t25394, t26550, t2061, t25402, t7056, t10073, t26544, t7064, t7384, t887, t689, t7399, t786);
    (t26551, t26554, t26555, t26557, t26558, t26560, t26561, t26563)
}
