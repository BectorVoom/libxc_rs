//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1115;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta269<F: Float>(t1358: F, t7492: F, t689: F, t2098: F, t786: F, t1364: F, t7250: F, t7257: F, t7260: F, t7267: F, t7253: F, t7265: F, t7272: F, t225: F, t2097: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7493, t7495, t7496, t7498, t7499, t7501, t7502, t7504, t7506) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1115::<F>(t1358, t7492, t689, t2098, t786, t1364, t7250, t7257, t7260, t7267, t7253, t7265, t7272);
        let (t7507, t7511) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1116::<F>(t225, t7506, t2097, t213);
    (t7493, t7495, t7496, t7498, t7499, t7501, t7502, t7504, t7506, t7507, t7511)
}
