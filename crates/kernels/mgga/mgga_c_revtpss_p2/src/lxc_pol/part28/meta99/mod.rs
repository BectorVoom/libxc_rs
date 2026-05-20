//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk630;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta99<F: Float>(t2251: F, t2275: F, t2258: F, t48: F, t59: F, t60: F, t239: F, t64: F, t2270: F, t44: F, t49: F, t56: F, t614: F, t617: F, t38: F, t45: F, t631: F, t78: F, t57: F, t635: F, t81: F, t633: F, t637: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2282, t2283, t2286, t2289, t2290, t2291) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk630::<F>(t2251, t2275, t2258, t48, t59, t60, t239, t64, t2270, t44, t49, t56, t614, t617);
        let (t2292, t2297, t2299, t2304, t2306, t2311) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk631::<F>(t2291, t38, t45, t631, t78, t57, t635, t81, t2251, t2258, t633, t637);
    (t2282, t2283, t2286, t2289, t2290, t2291, t2292, t2297, t2299, t2304, t2306, t2311)
}
