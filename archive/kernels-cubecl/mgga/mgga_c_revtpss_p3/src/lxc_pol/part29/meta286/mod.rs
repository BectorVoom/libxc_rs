//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1172;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta286<F: Float>(t1333: F, t3860: F, t4144: F, t4147: F, t30: F, t513: F, t33: F, t516: F, t2435: F, t3900: F, t212: F, t4066: F, t1358: F, t689: F, t3896: F, t9303: F, t1419: F, t785: F, t2439: F, t784: F, t209: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9597, t9599, t9605, t9617, t9632, t9634) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1172::<F>(t1333, t3860, t4144, t4147, t30, t513, t33, t516, t2435, t3900, t212, t4066);
        let (t9636, t9639, t9642, t9644, t9645, t9646) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1173::<F>(t1358, t9634, t689, t3896, t9303, t1419, t785, t2439, t784, t209);
    (t9597, t9599, t9605, t9617, t9632, t9636, t9639, t9642, t9644, t9645, t9646)
}
