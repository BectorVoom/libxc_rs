//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1102;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta266<F: Float>(t2089: F, t670: F, t2061: F, t212: F, t780: F, t689: F, t2062: F, t786: F, t789: F, t7023: F, t7031: F, t7034: F, t7041: F, t7026: F, t7039: F, t7046: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7378, t7384) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1102::<F>(t2089, t670, t2061, t212);
        let (t7385, t7387, t7388, t7390, t7391, t7393, t7394, t7396, t7398) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1103::<F>(t7384, t780, t689, t2062, t786, t789, t7023, t7031, t7034, t7041, t7026, t7039, t7046);
    (t7378, t7384, t7385, t7387, t7388, t7390, t7391, t7393, t7394, t7396, t7398)
}
