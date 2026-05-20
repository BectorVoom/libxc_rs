//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta469<F: Float>(t26292: F, t7289: F, t25969: F, t25975: F, t26002: F, t26010: F, t26012: F, t26021: F, t212: F, t7506: F, t1358: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26354, t26355, t26356) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1696::<F>(t26292, t7289, t25969, t25975, t26002, t26010, t26012, t26021, t212, t7506, t1358, t689);
    (t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26354, t26355, t26356)
}
