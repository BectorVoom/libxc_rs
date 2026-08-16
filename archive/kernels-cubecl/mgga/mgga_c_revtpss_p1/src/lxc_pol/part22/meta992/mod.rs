//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta992 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta992<F: Float>(t11354: F, t2881: F, t6120: F, t41382: F, t6113: F, t11358: F, t42731: F, t52011: F, t60927: F, t63468: F, t916: F, t41330: F, t41332: F, t63474: F, t63476: F, t63478: F, t63480: F, t63482: F, t63485: F, t63488: F, t63491: F) -> (F, F, F, F, F, F) {
        let (t63494, t63497, t63500, t63503, t63505, t63509) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3378::<F>(t11354, t2881, t6120, t41382, t6113, t11358, t42731, t52011, t60927, t63468, t916, t41330, t41332, t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491);
    (t63494, t63497, t63500, t63503, t63505, t63509)
}
