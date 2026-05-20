//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1119;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta270<F: Float>(t3182: F, t828: F, t2852: F, t357: F, t2251: F, t3093: F, t3109: F, t3096: F, t3091: F, t1020: F, t3105: F) -> (F, F, F, F, F, F, F, F) {
        let (t11703, t11705, t11706, t11707, t11710) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1119::<F>(t3182, t828, t2852, t357, t2251, t3093, t3109);
        let (t11711, t11712, t11714) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1120::<F>(t11710, t3096, t3091, t1020, t3105);
    (t11703, t11705, t11706, t11707, t11710, t11711, t11712, t11714)
}
