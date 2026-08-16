//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1708;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta459<F: Float>(t26249: F, t3908: F, t7507: F, t786: F, t1364: F, t2097: F, t3923: F, t543: F, t7301: F, t25937: F, t7282: F, t10073: F, t1426: F, t2098: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26251, t26252, t26253, t26255, t26257, t26260, t26261, t26263) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1708::<F>(t26249, t3908, t7507, t786, t1364, t2097, t3923, t543, t7301, t25937, t7282, t10073);
        let (t26264, t26265) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1709::<F>(t1426, t2098, t786);
    (t26251, t26252, t26253, t26255, t26257, t26260, t26261, t26263, t26264, t26265)
}
