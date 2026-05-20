//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta288<F: Float>(t10326: F, t36: F, t70: F, t2259: F, t627: F, t2291: F, t607: F, t363: F, t41: F, t46: F, t47: F, t2251: F, t606: F, sigma0: F) -> (F, F, F, F, F, F, F) {
        let (t10327, t10328, t10331, t10336, t10345, t10355, t10356) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1526::<F>(t10326, t36, t70, t2259, t627, t2291, t607, t363, t41, t46, t47, t2251, t606, sigma0);
    (t10327, t10328, t10331, t10336, t10345, t10355, t10356)
}
