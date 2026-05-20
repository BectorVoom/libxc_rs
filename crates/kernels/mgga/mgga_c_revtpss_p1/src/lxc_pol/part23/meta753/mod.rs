//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta753<F: Float>(t52035: F, t52037: F, t11223: F, t1678: F, t1041: F, t11262: F, t4868: F, t3201: F, t4794: F, t4798: F, t343: F, t44: F, t816: F) -> (F, F, F, F, F, F, F) {
        let (t53252, t53253, t53281, t53294, t53300, t53318, t53320) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2542::<F>(t52035, t52037, t11223, t1678, t1041, t11262, t4868, t3201, t4794, t4798, t343, t44, t816);
    (t53252, t53253, t53281, t53294, t53300, t53318, t53320)
}
