//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2033;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta601<F: Float>(t94589: F, t97814: F, t2435: F, t27965: F, t14090: F, t26054: F, t25894: F, t97676: F, t97680: F, t14110: F, t94901: F, t10073: F, t1903: F, t2029: F, t25929: F, t1904: F, t25912: F, t689: F, t1385: F, t7910: F, t14104: F, t94725: F, t1358: F, t2439: F, t785: F, t7925: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97815, t97823, t97825, t97838, t97843, t97847) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2033::<F>(t94589, t97814, t2435, t27965, t14090, t26054, t25894, t97676, t97680, t14110, t94901, t10073, t1903, t2029, t25929);
        let (t97869, t97875, t97882, t97894, t97899) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2034::<F>(t1904, t25912, t689, t1385, t7910, t14104, t94725, t1358, t2439, t785, t2435, t7925);
    (t97815, t97823, t97825, t97838, t97843, t97847, t97869, t97875, t97882, t97894, t97899)
}
