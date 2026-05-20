//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta541<F: Float>(t25375: F, t95628: F, t136: F, t137: F, t2061: F, t10505: F, t93377: F, t7406: F, t9288: F, t7064: F, t10073: F, t25308: F, t26554: F) -> (F, F, F, F, F, F, F) {
        let (t95722, t95725, t95726, t95727, t95730, t95732, t95740) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1852::<F>(t25375, t95628, t136, t137, t2061, t10505, t93377, t7406, t9288, t7064, t10073, t25308, t26554);
    (t95722, t95725, t95726, t95727, t95730, t95732, t95740)
}
