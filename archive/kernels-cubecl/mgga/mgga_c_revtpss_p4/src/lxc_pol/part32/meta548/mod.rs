//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta548<F: Float>(t7289: F, t96282: F, t26277: F, t94776: F, t25950: F, t26292: F, t25904: F, t96245: F, t94471: F, t94473: F, t94476: F, t94483: F) -> (F, F, F, F, F, F, F, F) {
        let (t96284, t96287, t96289, t96298, t96321, t96322, t96323, t96326) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1863::<F>(t7289, t96282, t26277, t94776, t25950, t26292, t25904, t96245, t94471, t94473, t94476, t94483);
    (t96284, t96287, t96289, t96298, t96321, t96322, t96323, t96326)
}
