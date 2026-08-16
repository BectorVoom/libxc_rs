//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta421<F: Float>(t1568: F, t785: F, t780: F, t2439: F, t212: F, t4469: F, t689: F, t1579: F, t2769: F, t886: F, t252: F, t2782: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14472, t14473, t14474, t14476, t14477, t14479, t14480, t14481, t14482, t14484) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2031::<F>(t1568, t785, t780, t2439, t212, t4469, t689, t1579, t2769, t886, t252, t2782);
    (t14472, t14473, t14474, t14476, t14477, t14479, t14480, t14481, t14482, t14484)
}
