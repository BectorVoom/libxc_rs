//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta537<F: Float>(t10505: F, t95725: F, t93377: F, t7406: F, t9288: F, t7064: F, t25411: F, t95593: F, t10073: F, t25308: F, t26554: F, t7399: F, t786: F, t867: F) -> (F, F, F, F, F, F, F) {
        let (t95726, t95727, t95730, t95732, t95733, t95740, t95743) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1869::<F>(t10505, t95725, t93377, t7406, t9288, t7064, t25411, t95593, t10073, t25308, t26554, t7399, t786, t867);
    (t95726, t95727, t95730, t95732, t95733, t95740, t95743)
}
