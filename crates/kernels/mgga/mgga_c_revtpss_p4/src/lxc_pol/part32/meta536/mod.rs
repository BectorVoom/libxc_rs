//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1845;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta536<F: Float>(t1419: F, t786: F, t25877: F, t2453: F, t25949: F, t25898: F, t112: F, t843: F, t239: F, t655: F, t665: F, t2339: F, t624: F, t10208: F, t68: F, t25081: F, t7234: F, t1923: F, t26204: F, t6977: F, t1927: F, t72: F, t26205: F, t6954: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94890, t94913, t94921, t94973, t94975, t94976, t94978) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1845::<F>(t1419, t786, t25877, t2453, t25949, t25898, t112, t843, t239, t655, t665, t2339, t624);
        let (t94982, t95088, t95246, t95253, t95255) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1846::<F>(t10208, t68, t25081, t7234, t1923, t26204, t6977, t1927, t72, t843, t26205, t6954);
    (t94890, t94913, t94921, t94973, t94975, t94976, t94978, t94982, t95088, t95246, t95253, t95255)
}
