//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta535<F: Float>(t26474: F, t686: F, t72: F, t7058: F, t7064: F, t25387: F, t95571: F, t11050: F, t26497: F, t92975: F, t92988: F, t92995: F) -> (F, F, F, F, F, F, F) {
        let (t95645, t95647, t95649, t95651, t95666, t95671, t95673) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1867::<F>(t26474, t686, t72, t7058, t7064, t25387, t95571, t11050, t26497, t92975, t92988, t92995);
    (t95645, t95647, t95649, t95651, t95666, t95671, t95673)
}
