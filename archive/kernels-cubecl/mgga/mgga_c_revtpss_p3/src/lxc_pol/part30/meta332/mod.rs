//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1338;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta332<F: Float>(t2645: F, t2723: F, t10115: F, t253: F, t233: F, t2760: F, t869: F, t689: F, t2777: F, t2789: F, t2439: F, t2435: F, t2790: F, t2778: F, t9303: F, t871: F, t9292: F, t72: F, t686: F, t874: F, t251: F, t9646: F, t22: F, t780: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10943, t10948, t10961, t10964, t10966) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1338::<F>(t2645, t2723, t10115, t253, t233, t2760, t869, t689, t2777, t2789, t2439, t2435, t2790);
        let (t10969, t10971, t10974, t10981, t10982) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1339::<F>(t2778, t9303, t871, t9292, t2760, t72, t686, t874, t251, t9646, t22, t780);
    (t10943, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t10981, t10982)
}
