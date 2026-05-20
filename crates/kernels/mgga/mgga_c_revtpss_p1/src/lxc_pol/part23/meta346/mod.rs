//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta346<F: Float>(t14495: F, t2797: F, t2782: F, t1558: F, t860: F, t231: F, t2783: F, t251: F, t4423: F, t10073: F, t4496: F, t10542: F, t4500: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14496, t14498, t14502, t14504, t14506, t14507, t14509, t14511, t14512, t14518) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1649::<F>(t14495, t2797, t2782, t1558, t860, t231, t2783, t251, t4423, t10073, t4496, t10542, t4500);
    (t14496, t14498, t14502, t14504, t14506, t14507, t14509, t14511, t14512, t14518)
}
